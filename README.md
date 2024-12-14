# Rust Raw Pointer Vector Modification Leading to Undefined Behavior

This repository demonstrates a common error in Rust: modifying a vector through a raw pointer after the vector's length or capacity has changed. This leads to undefined behavior, meaning the program can crash, produce incorrect results, or behave unpredictably.

The `bug.rs` file contains the buggy code. The `bugSolution.rs` file provides a corrected version.

This example highlights the importance of careful memory management and understanding the implications of using raw pointers in Rust.