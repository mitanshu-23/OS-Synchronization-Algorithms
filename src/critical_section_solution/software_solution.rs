// 3 needs to be strictly followed
//Mutual Exclusion
//Progress
//Bounded Waiting

use std::{thread, time::Instant};

static mut COUNTER: i32 = 0;
static mut LOCK: bool = false;

//Mutual Exclusion not satisfied
pub fn using_lock() {
    let mut handles = vec![];

    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            for _ in 0..1000 {
                unsafe {
                    while LOCK {}
                    LOCK = true;
                    COUNTER += 1;
                    LOCK = false;
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        let value = COUNTER; // copy value to avoid creating a shared reference
        println!("Final Counter Using LOCK = {}", value);
    }
}

//Works fine but progress not satisfied: Initally onlyone process can enter Critical Section. Executein strict alteration
static mut TURN: i32 = 0;
pub fn using_turn() {
    let mut handles = vec![];

    let start = Instant::now();
    for ind in 0..=1 {
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                unsafe {
                    while TURN != ind {}
                    COUNTER += 1;
                    TURN = (TURN + 1) % 2;
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    unsafe {
        let value = COUNTER; // copy value to avoid creating a shared reference
        println!("Final Counter Using TURN= {} in duration= {:?}", value, duration);
    }
}

static mut FLAG: [bool; 2] = [false, false];
static mut TURNP: i32 = 0;
pub fn peterson_solution() {
    let mut handles = vec![];

    let start = Instant::now();
    for ind in 0..=1 {
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                unsafe {
                    FLAG[ind] = true;
                    TURNP = (ind as i32 + 1) % 2;
                    while FLAG[(ind + 1) % 2] && TURNP == ((ind + 1) % 2) as i32 {}
                    COUNTER += 1;
                    FLAG[ind] = false;
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let duration = start.elapsed();
    unsafe {
        let value = COUNTER; // copy value to avoid creating a shared reference
        println!(
            "Final Counter Using Peterson's Solution= {} in duration= {:?}",
            value, duration
        );
    }
}
