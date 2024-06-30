//? Entry point and boot procedures.

#pragma once

// Prepare kernel for boot. This function depends on the configured bootloader.
void kernel_boot();

// Main entry point. Normal code execution starts here.
void kernel_main();
