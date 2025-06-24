# Command line arguments

You can provide the kernel with a list of command line arguments.
They are seperated by spaces and formatted like so:

```
argument=value argument2="other value"
```

If an argument is a boolean type, the following values are accepted:
`true`/`yes`/`on`/`1` or `false`/`no`/`off`/`0` respectively.

String arguments can be escaped with double quotes (`" "`).

## List of arguments

| Name   | Type   | Default  | Description                                                        |
| ------ | ------ | -------- | ------------------------------------------------------------------ |
| init   | string | `/init`  | The file path to the init process to start                         |
| smp    | usize  | All CPUs | If set, only initializes the specified amount of CPUs              |
| acpi   | bool   | true     | On supported platforms, uses ACPI to configure the machine         |
| openfw | bool   | true     | On supported platforms, uses device trees to configure the machine |
| pci    | bool   | true     | Configures the PCI subsystem                                       |
| tsc    | bool   | false    | If true, enables the TSC as a clock source                         |
