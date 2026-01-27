// By default, Rust prevents race conditions in safe code.
// However, you can simulate a race condition by using an `unsafe` block.

// Our goal is to demonstrate how to solve or mitigate this race condition **while still using unsafe Rust**.
// That is, the solution will be applied on top of the unsafe code, without removing the `unsafe` block
// or relying on Rust's safety guarantees.

fn main() {
    println!("Hello, Welcome to OS-Synchronization-Algorithms");
}
