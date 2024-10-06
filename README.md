# menix
A minimal and expandable Unix-like operating system.

This repository contains only the kernel itself. You can run it on its own in
an emulator like QEMU, but it won't do much.

Unlike microkernel or monolithic designs, menix chooses to only keep core subsystems in the kernel,
while the drivers using these subsystems.

> [!Note]
> The project is currently in a pre-alpha stage of development and neither stable nor ready to use.

## Getting started
```sh
git clone https://github.com/menix-os/kernel --recursive
cargo build --release
```

For more info, see [Building](doc/building.md)

## Contributing
Contributions are _always_ welcome!
First, please read the [contributing guide](doc/contributing.md) to make sure
your code fits to the rest of the project.
Open an issue or pull request with the appropriate template and submit your changes.

## Documentation
More in-depth documentation can be found in the [kernel-docs](https://github.com/menix-os/kernel-docs) repository.
