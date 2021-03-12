pub fn fib(num: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;

    for _ in 1..=num {
        let tmp = a + b;
        a = b;
        b = tmp;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_fib_0() {
        // GIVEN
        let n = 0;

        // WHEN
        let res = fib(n);

        // THEN
        assert_eq!(res, 0);
    }

    #[test]
    fn it_should_return_fib_1() {
        // GIVEN
        let n = 1;

        // WHEN
        let res = fib(n);

        // THEN
        assert_eq!(res, 1);
    }

    #[test]
    fn it_should_return_fib_4() {
        // GIVEN
        let n = 4;

        // WHEN
        let res = fib(n);

        // THEN
        assert_eq!(res, 3);
    }
}
