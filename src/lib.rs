#![allow(dead_code)]

pub mod c {
    extern "C" {
        pub fn add(a: i32, b: i32) -> i32;
        pub fn fib(a: i32) -> i32;
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn text_export() {
        assert_eq!(2, unsafe { super::c::add(1, 1) });
        assert_eq!(2, super::add(1, 1));

        assert_eq!(55, unsafe { super::c::fib(10) });
        assert_eq!(55, super::fib(10));
    }
}
