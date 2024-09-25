use anyhow::Context;
use clap::Parser;
use decompress::{ExtractOpts, ExtractOptsBuilder};
use serde::Deserialize;
use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader, Read, Write},
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Build all packages in `path`.
    #[arg(long)]
    all: bool,

    /// Target install path.
    #[arg(long, default_value = "build/sysroot")]
    sysroot: PathBuf,

    /// Directory where to store downloaded sources.
    #[arg(long, default_value = "build/source")]
    source_path: PathBuf,

    /// Target architecture.
    #[arg(long, default_value = std::env::consts::ARCH)]
    target: String,

    /// Path to the package to build.
    path: PathBuf,
}

#[derive(Deserialize, Clone, Debug)]
struct Package {
    info: PackageInfo,
    source: PackageSource,
    build: PackageScript,
    install: PackageScript,
}

#[derive(Deserialize, Clone, Debug)]
struct PackageInfo {
    name: String,
    version: String,
    archs: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
enum PackageSource {
    Archive {
        archive: PathBuf,
    },
    Git {
        repo: String,
        branch: Option<String>,
    },
    Local {
        path: PathBuf,
    },
}

#[derive(Deserialize, Clone, Debug)]
struct PackageScript {
    script: String,
}

fn run_command(mut command: Command) -> anyhow::Result<()> {
    let status = command
        .status()
        .context(format!("failed to run command: {:?}", command))?;

    if !status.success() {
        anyhow::bail!("failed to run {:?}: exited with status {}", command, status);
    }

    Ok(())
}

fn run_command_stdin(mut command: Command, stdin_data: &[u8]) -> anyhow::Result<()> {
    command.stdin(Stdio::piped());

    let mut child = command
        .spawn()
        .context(format!("failed to spawn {:?}", command))?;

    if let Some(ref mut stdin) = child.stdin {
        stdin
            .write_all(stdin_data)
            .context(format!("failed to write stdin of {:?}", command))?;
    } else {
        anyhow::bail!("failed to find stdin of {:?}", command);
    }

    let status = child
        .wait()
        .context(format!("failed to run {:?}", command))?;

    if !status.success() {
        anyhow::bail!("failed to run {:?}: exited with status {}", command, status)
    }

    Ok(())
}

/// `base_dir` must be the package dir, not the package file
fn step_source(args: &Args, base_dir: &Path, package: &Package) -> anyhow::Result<()> {
    let pkg_path = base_dir.join("pkg.toml");
    let source_path = args.source_path.join(&package.info.name);
    let source_marker_path = source_path.join(".source");

    let pkg_meta = pkg_path.metadata().context("Failed to stat package file")?;

    let should_get_pkg_source = if let Ok(source_marker_meta) = source_marker_path.metadata() {
        if pkg_meta.modified()? > source_marker_meta.modified()? {
            println!("Package file modified, removing existing sources");
            fs::remove_dir_all(&source_path).context("Failed to remove existing source dir")?;
            true
        } else {
            false
        }
    } else {
        true
    };

    if should_get_pkg_source {
        match &package.source {
            PackageSource::Git { repo, branch } => {
                let mut clone_cmd = Command::new("git");
                clone_cmd.arg("clone");

                if let Some(branch_name) = branch {
                    clone_cmd.args(vec!["--branch", branch_name]);
                }

                clone_cmd.arg(repo).arg(source_path);
                run_command(clone_cmd).context("Failed to clone package repository")?;
            }
            PackageSource::Archive {
                archive: archive_path,
            } => {
                let archive_path = resolve_path(&archive_path, base_dir);

                decompress_archive(&archive_path, &source_path)
                    .context(format!("Failed to open archive: {:?}", archive_path))?;
            }
            PackageSource::Local { path: local_path } => {
                let local_path = resolve_path(&local_path, base_dir);

                copy_dir_all(local_path, source_path)
                    .context("Failed to copy local directory to build directory")?;
            }
        }
        touch(&source_marker_path).context("Failed to update source marker file (.source)")?;
    }

    Ok(())
}

fn resolve_path(path: &Path, base_dir: &Path) -> PathBuf {
    // handle absolute and relative paths
    if path.is_absolute() {
        path.to_owned()
    } else {
        base_dir.join(path)
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Like the linux `touch` command
fn touch(file: &Path) -> anyhow::Result<()> {
    File::options()
        .create(true)
        .write(true)
        .open(file)
        .context("Failed to open file")
        .map(|_| ())
}

fn decompress_archive(archive: &Path, decompress_to: &Path) -> anyhow::Result<()> {
    decompress::decompress(
        &archive,
        &decompress_to,
        &ExtractOptsBuilder::default()
            .build()
            .expect("Default compress builder should always work"),
    )
    .context("Failed to decompress archive")?;

    Ok(())
}

fn step_build(args: &Args, base_dir: &Path, package: &Package) -> anyhow::Result<()> {
    let build_cmd = {
        let mut cmd = Command::new("bash");
        cmd
    };

    let build_env = include_str!("build_env.sh");
    let full_script = format!("{}\n{}", build_env, &package.build.script);
    run_command_stdin(build_cmd, full_script.as_bytes())?;
    Ok(())
}

fn step_install(args: &Args, path: &Path, package: &Package) -> anyhow::Result<()> {
    todo!("implement install step")
}

fn make_pkg(args: &Args, path: &Path) -> anyhow::Result<()> {
    let (pkg_base_dir, pkg_file_path) = if path.is_dir() {
        (path.to_owned(), path.join("pkg.toml"))
    } else {
        (
            path.parent()
                .map(|x| x.to_owned())
                // When the path has no parent (we are in the same dir as the pkg file),
                // we set . as the base dir
                .unwrap_or(PathBuf::from(".")),
            path.to_owned(),
        )
    };

    let package_file = fs::read_to_string(&pkg_file_path).context("Failed to read package file")?;
    let package =
        toml::from_str::<Package>(&package_file).context("Failed to deserialize package file")?;

    // Check if the package is architecture dependent. If yes, check if we're targeting this architecture.
    if !package.info.archs.is_empty() && !package.info.archs.contains(&args.target) {
        anyhow::bail!("Arch not targetet");
    }

    println!(
        "Building \"{}\" ({})",
        package.info.name, package.info.version
    );

    // Get source files.
    println!("Getting Sources...");
    step_source(args, &pkg_base_dir, &package)?;

    // Build package.
    step_build(args, &pkg_base_dir, &package)?;

    // Install package to build root.
    step_install(args, &pkg_base_dir, &package)?;

    return Ok(());
}

fn try_run_make_pkg(args: &Args, path: &Path) -> anyhow::Result<()> {
    make_pkg(&args, &path).context(format!("Failed to build package at: {:?}", &path))
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.all {
        // Build all packages in the path directory.
        for entry in args
            .path
            .read_dir()
            .context("Failed to read target directory")?
        {
            let entry = entry.context("Failed to read target directory entry")?;
            let file_type = entry
                .file_type()
                .context("Failed to stat target dir entry")?;
            if file_type.is_dir() {
                let path = entry.path();
                try_run_make_pkg(&args, &path)?;
            }
        }
    } else {
        try_run_make_pkg(&args, &args.path)?;
    }

    Ok(())
}
