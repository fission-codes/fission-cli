pub fn get_fibinaci(n:u32) -> u64{
    if n <= 1 {
        return n as u64;
    }
    return get_fibinaci(n - 1) + get_fibinaci(n - 2);
}