/*
   This file is currently dead code. Please remove if it continue to be dead code.
*/

pub fn get_fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    return get_fibonacci(n - 1) + get_fibonacci(n - 2);
}
