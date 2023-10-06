Comparison of DLL built with Rust vs MSVC. The "server" in each case has a lot of similar functions to test how well the compiler can fold and reduce code size.

For Rust, `-C prefer-dynamic` is used to reduce the overhead of the Rust standard library. Unfortunately, `prefer-dynamic` is not compatible with `lto = true` or `panic = "abort"` on Windows. COMDAT folding also does not appear to kick in at all, so as the binary moves from "hello world" to "real world" the size improvements from `prefer-dynamic` disappears. This is exacerbated by the fact that LLVM does not implement `FrameHandler4`. 

Comparing "x64 Release" builds of C++ and Rust with the latter compiled with `cargo build -p server --release` here are the results:

| | Rust | C++ |
|-|------|-----|
| Minimal code | 29,696 | 40,960 |
| Lots of foldable code | 151,552 | 86,016 |

Note that the Rust build script calls `midlrt.exe` so be sure to run it from a Visual Studio tools command prompt. 