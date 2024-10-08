# Debugging the kernel
After starting the full image in QEMU with DEBUG=1, emulation will be stopped to
allow you to attach a debugger. You can use the following methods to attach to
QEMU. Note that these require you to have built the kernel in debug mode.

## VSCode (CodeLLDB)
To debug with VSCode/VSCodium, install the CodeLLDB extension.
Then simply press `Run > Start Debugging` or press F5 and select the correct
debug profile for your architecture.

## LLDB
To debug with LLDB, run the following commands:
```
$ lldb
(lldb) target create kernel/target/x86_64/debug/menix
(lldb) gdb-remote localhost:1234
```
