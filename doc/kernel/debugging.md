# Debugging

This document explains how to debug the kernel using LLDB.

## Kernel
To load the symbols for the kernel, run:
```
target create build/bin/menix
```
This should automatically create all symbol mappings.


## Dynamic Modules
Built-in modules are automatically included when debugging the kernel,
since they're part of the same binary.
This is not the case with dynamic modules! They are relocated at load time,
so you will have to manually provide the virtual base address where the module
was loaded. This address is printed to the console via `kmesg` during load time.

For every module you want to debug, you'll have to run:
```
target modules add <path to module>
```
Then, after it's loaded:
```
target modules load --file <file name> --slide <loaded address>
```

## Debugging with VSCode
If you're debugging with VSCode and CodeLLDB, you can use the following example
for your `.vscode/launch.json` (Note that you will still have to run the command
above manually before resuming execution):
```json
{
	"version": "0.2.0",
	"configurations": [
		{
			"type": "lldb",
			"request": "custom",
			"name": "Debug",
			"targetCreateCommands": [
				// Kernel, mandatory
				"target create ${workspaceFolder}/build/bin/menix",
				// Optional dynamic modules
				"target modules add ${workspaceFolder}/build/bin/mod/<name>"
			],
			"processCreateCommands": [
				"gdb-remote localhost:1234"
			]
		}
	]
}
```