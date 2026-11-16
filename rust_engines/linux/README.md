# Optional native Linux engines

Remus5 through Remus8 have independent Linux source trees and x86-64 ELF
executables in this folder. The workshop remains Windows-focused. The Windows
sources, executables, and guides live separately one directory above.

Build on x86-64 Linux with Rust and a C build toolchain:

```bash
bash utils/linux/build_linux_engines.sh
```

Run this command from the repository root. Each Linux folder is also a standalone
Cargo project. The supplied executables target glibc 2.28 or later. These are
ordinary release builds; Windows speed and Elo measurements do not establish
Linux performance. Native Linux runtime validation is required before use.

Remus6 through Remus8 allocate aligned, zero-filled table memory with Rust's
allocator. There are no Windows API calls or conditional platform branches.
The chess algorithms match the corresponding Windows versions. Maintain
algorithm changes in both source trees.

The optional setup utilities live in `utils/linux`. Run them with `bash` and
`--help` for usage. Student cleanup is only for a fresh disposable clone;
experimenters who want completed examples should keep their engine modules.
