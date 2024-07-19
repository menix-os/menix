# menix

A minimal and expandable Unix-like operating system.

This is a hobby project to learn OS development.
The goal is to have a system that boots on real hardware with decent user
interface, shell and package manager.

The kernel itself uses a microkernel architecture and only handles:
- Booting
- Memory management
- HAL
- IPC

The project is written in C23.

## Building from source
See [Building](doc/building.md)

## Contributing
First, please read the [contributing guide](doc/contributing.md).

- [Kernel Documentation](doc/kernel/readme.md)
