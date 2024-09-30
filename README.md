# menix

A minimal and expandable Unix-like operating system.

menix uses a hybrid kernel design. The kernel itself handles:
- Booting
- Hardware Abstraction
- Interprocess Communication
- Scheduling
- Memory Management
- Virtual File System

Unlike classic microkernel/monolithic designs, menix chooses to only keep core subsystems in the kernel,
while the drivers using these subsystems

## Building from source
```sh
cargo build --release
```
See [Building](doc/building.md)

> [!Note]
> This is a hobby project to learn OS development.
> The project is currently in a pre-alpha stage of development and neither stable nor ready to use.

## Contributing
Contributions are _always_ welcome!
First, please read the [contributing guide](doc/contributing.md) to make sure
your code fits to the rest of the project.
Open an issue or pull request with the appropriate template and submit your changes.

## Documentation
More in-depth documentation can be found in the [kernel-docs](https://github.com/menix-os/kernel-docs) repository.
