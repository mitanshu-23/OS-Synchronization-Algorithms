// This example demonstrates the effect of a race condition in Rust.

// Rust's ownership model is specifically designed to prevent data races in safe code.

// Race Condition: Occurs when two or more threads access the same memory location simultaneously,
// at least one of them is a write, and there is no proper synchronization.

// Rust prevents data races in safe code. To simulate a race condition, we need to use unsafe Rust.

use std::thread;

static mut COUNTER: i32 = 0;
// In the code below, we deliberately bypass Rust's safety guarantees using `unsafe`.
// This allows the code to behave unpredictably, and we acknowledge this risk.
//
// We spawn 10 threads, each incrementing the value of COUNTER. Since COUNTER is a
// non-atomic global variable, and the program is multithreaded, we cannot predict
// the order in which threads execute or are preempted.
//
// The operation `COUNTER += 1` is **not atomic**; it consists of multiple steps:
// 1. Load the current value of COUNTER into a register.
// 2. Increment the value.
// 3. Store the incremented value back into COUNTER.
//
// If a thread is preempted after reading COUNTER but before storing the incremented value,
// another thread may also read the same original value. When both threads proceed, they will
// overwrite each other's updates, resulting in a missed increment. Essentially, **the thread
// that writes last "wins"**, determining the value stored in memory at that moment.
//
// Therefore, the final value of COUNTER may be **less than 10,000** (10 threads × 1,000 increments),
// depending on the timing of thread execution.

pub fn test() {
    let mut handles = vec![];

    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            for _ in 0..1000 {
                unsafe {
                    COUNTER += 1;
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        let value = COUNTER; // copy value to avoid creating a shared reference
        println!("Final Counter = {}", value);
    }
}

// If you want to see how Rust prevents data races, you can uncomment the code below.
//
// Rust will not allow mutation of a `static mut` variable outside an `unsafe` block.
// This enforces that the programmer is responsible for ensuring correct behavior when
// mutating global state across threads.
//
// Uncommenting the code below without `unsafe` will result in a compile-time error:

// pub fn test() {
//     let mut handles = vec![];

//     for _ in 0..10 {
//         handles.push(thread::spawn(|| {
//             for _ in 0..1000 {
//                 // ❌ This will not compile: mutable access to static variable must be unsafe
//                 COUNTER += 1;
//             }
//         }));
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }
// }
