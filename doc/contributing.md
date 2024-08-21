# Contributing
This document explains how to format and submit contributions to
the `menix` project.

## General rules
- **NO** discriminatory behavior, everyone should be welcome to contribute.
- Every submission should solve exactly **ONE** problem.
- Your submission **MUST** be written by you.
- Your code **MUST** be licensed under the project's main license, or
  one that has less restrictions.
- Currently, the following licenses are supported:
	- LGPL-2.1-or-later
	- BSD-2-Clause
	- BSD-3-Clause
	- MIT

At the top of each file, include the following section. It should contain a
very brief summary of the file's purpose, followed by the SPDX identifier of
the code. If it is omitted, it's assumed to be the same as the project's
main license.

<!-- REUSE-IgnoreStart -->
  **Example:**
  ```c
  // Implementation for a cool feature
  // SPDX-License-Identifier: BSD-3-Clause
  ```
<!-- REUSE-IgnoreEnd -->

## Coding style
The following section explains how code is expected to be formatted in order
to make it easier to read for other people. Use the `.clang-format` file in the
project root to check if your code complies with these guidelines.

- Always use tabs for indentation.
  > **Rationale:** Different people prefer different indentations. Having an
  > adjustable indentation fixes this.
- Tabs have a width of 4 characters.
- All code should be less than 120 characters in width.
  > **Note:** Long paragraphs of doc strings should be around 80 characters
  > in width to make it easier to read.
- All code should end on an empty newline.
- Scope braces `{ }` begin and end on a new line.
  > **Rationale:** When scanning the screen vertically with your eyes,
  > it's much faster to see where scope braces open and close.
- Write comments and messages in present tense and easy to understand English.
- Don't just document what your code does, explain why it has to exist.
  > **Rationale:** This helps avoid useless code and helps less experienced
  > programmers learn directly from the code.
- Split your code into paragraphs where it makes sense and comment what
  each one does.
- Always use single line (aka C++ style) comments.
- Headers should always use `#pragma once` instead of header guards.
- Headers should first include standard libary headers, then `menix` headers,
  then relative headers.
- **DO NOT** include out of tree headers like `#include "../../random.h"`,
  always use `#include <path/to/random.h>` instead.
- For primitives, always use standard types like `i32` and `u8`
  over built-in types like `int` or `unsigned char` (exception here is `char`).
- Avoid `i64`, `u64`, `i128` and `u128`, as they might not be available on all platforms.
  > **Note:** Consider using `isize` and `usize` instead.
- Avoid floating-point types.
- Avoid using `this` as variable or identifier names, use `self` instead.
- Function declarations are always in one line.
- Structs and unions are to be declared via `typedef`. They should be anonymous.
- Multi-line `typedef`s have the alias name and closing brace on the same line.
- Pointer stars are part of the type and should be written as `Type*`.
- Avoid raw and inline assembly at all costs to keep the codebase portable.
- Prefer clear and descriptive variable names over short ones.
- Local variables and fields use `snake_case`.
- Defines use `SCREAMING_CASE`.
- Functions use `snake_case`.
- Types use `PascalCase`.
- Function Types should have a `Fn` suffix.
- Use the `const` keyword often to communicate intent with function parameters.
