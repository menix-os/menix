use clap::Parser;
use serde::Deserialize;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Build all packages in `path`.
    #[arg(long, default_value_t = false)]
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
    Tar { tar: String },
    Git { git: String, branch: Option<String> },
    Local { local: PathBuf },
}

#[derive(Deserialize, Clone, Debug)]
struct PackageScript {
    script: String,
}

fn run_command(mut command: Command) -> Result<(), String> {
    let status = command
        .status()
        .map_err(|err| format!("failed to run {:?}: {}\n{:#?}", command, err, err))?;

    if !status.success() {
        return Err(format!(
            "failed to run {:?}: exited with status {}",
            command, status
        ));
    }

    Ok(())
}

fn run_command_stdin(mut command: Command, stdin_data: &[u8]) -> Result<(), String> {
    command.stdin(Stdio::piped());

    let mut child = command
        .spawn()
        .map_err(|err| format!("failed to spawn {:?}: {}\n{:#?}", command, err, err))?;

    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(stdin_data).map_err(|err| {
            format!(
                "failed to write stdin of {:?}: {}\n{:#?}",
                command, err, err
            )
        })?;
    } else {
        return Err(format!("failed to find stdin of {:?}", command));
    }

    let status = child
        .wait()
        .map_err(|err| format!("failed to run {:?}: {}\n{:#?}", command, err, err))?;

    if !status.success() {
        return Err(format!(
            "failed to run {:?}: exited with status {}",
            command, status
        ));
    }

    Ok(())
}

fn step_source(args: &Args, path: &Path, package: &Package) -> Result<(), Box<dyn Error>> {
    let mut pkg_path = path.to_path_buf();
    pkg_path.push("pkg.toml");
    let mut source_path = args.source_path.clone();
    source_path.push(&package.info.name);
    source_path.push(".source");

    // Download sources if they weren't already.
    if !source_path.exists()
        || pkg_path.metadata()?.modified()? > source_path.metadata()?.modified()?
    {
        match &package.source {
            PackageSource::Git { git, branch } => {
                let mut source_cmd = Command::new("git");
                let mut clone_path = args.source_path.clone();
                clone_path.push(&package.info.name);
                source_cmd.arg("clone");
                if let Some(branch_name) = branch {
                    source_cmd.args(vec!["--branch", branch_name]);
                }
                source_cmd.arg(git).arg(clone_path);
                run_command(source_cmd)?;
            }
            PackageSource::Tar { tar } => {
                todo!()
            }
            PackageSource::Local { local } => {
                todo!()
            }
        }
        File::create_new(source_path)?;
    }

    Ok(())
}

fn step_build(args: &Args, path: &Path, package: &Package) -> Result<(), Box<dyn Error>> {
    let build_cmd = {
        let mut cmd = Command::new("bash");
        cmd
    };

    let build_env = include_str!("build_env.sh");
    let full_script = format!("{}\n{}", build_env, &package.build.script);
    run_command_stdin(build_cmd, full_script.as_bytes())?;
    Ok(())
}

fn step_install(args: &Args, path: &Path, package: &Package) -> Result<(), Box<dyn Error>> {
    todo!()
}

fn make_pkg(args: &Args, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut pkg_path = path.to_path_buf();
    pkg_path.push("pkg.toml");

    // Read package file.
    let mut f = File::open(pkg_path).unwrap();
    let mut buf = String::new();
    _ = f.read_to_string(&mut buf);
    let package = toml::from_str::<Package>(&buf).unwrap();

    // Check if the package is architecture dependent. If yes, check if we're targeting this architecture.
    if package.info.archs.len() > 0 && !package.info.archs.contains(&args.target) {
        return Ok(());
    }

    println!(
        "Building \"{}\" ({})",
        package.info.name, package.info.version
    );

    // Get source files.
    step_source(args, path, &package)?;

    // Build package.
    step_build(args, path, &package)?;

    // Install package to build root.
    step_install(args, path, &package)?;

    return Ok(());
}

fn main() {
    let args = Args::parse();

    // Build all packages in the path directory.
    if args.all {
        for entry in args.path.read_dir().unwrap() {
            make_pkg(&args, &entry.unwrap().path()).unwrap();
        }
    } else {
        make_pkg(&args, &args.path).unwrap();
    }
}
