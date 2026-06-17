![CI Test](https://github.com/lmeller-git/libtinyos/actions/workflows/rust_tests.yml/badge.svg?branch=main)

# libtinyos

The core userspace libraries and ABI wrappers for tinyOS.

## Overview

This repository contains the foundational libraries used to write userspace applications for **[tinyOS](https://github.com/lmeller-git/tinyos)**.
Instead of forcing developers to interact directly with the raw ABI, this workspace provides somewhat ergonomic abstractions for system resources, I/O, and graphical interfaces.

This repository is split into two primary crates, both of which expose C-compatible interfaces:

### 1. `libtinyos`
The standard interface library for tinyOS.
* Acts as a wrapper around the raw `tinyos_abi`.
* Handles fundamental OS interactions, including standard input/output streams, file system access, and basic process and threading management.
* **C Bindings:** Exposes a small C API in, which may be found in `libtinyos/libtinyos.h`, allowing traditional C programs to interface with the core OS runtime.

### 2. `libtinygraphics`
The dedicated graphics and windowing library.
* Interfaces directly with tinyOS's exposed hardware framebuffers.
* Allows easy management of raw framebuffers.
* Features out-of-the-box integration with rendering frameworks like **Ratatui** for Rust-based terminal UIs.
* **C Bindings:** Provides drawing and framebuffer management bindings for applications compiled from C. May be found in `tinygraphics/tinygraphics.h`.

---

## FFI & C Interoperability

To bridge the gap between Rust and standard systems programming languages, both libraries utilize Rust's Foreign Function Interface (FFI).

The exposed C bindings handle the necessary `unsafe` boundaries, data layout conversions, and `extern "C"` linkage symbols required to write, compile, and run native C applications or ports on top of tinyOS.

---

## Usage & Examples

These libraries are intended to be consumed when building programs for the tinyOS ecosystem. Because they rely heavily on the custom OS ABI, they currently only compile for tinyOS targets.

To see exactly how to link, initialize, and utilize these libraries (in Rust, C and ASM), check out the example program recipes in the **[tinyosprograms](https://github.com/lmeller-git/tinyosprograms)** repository.

### Quick Links

* **[tinyOS Core](https://github.com/lmeller-git/tinyos)** — The main kernel, build orchestrator, and architectural documentation.
* **[tinyosprograms](https://github.com/lmeller-git/tinyosprograms)** — Example applications, build templates, reference implementations and applications included in the tinyOS image.

