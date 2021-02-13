pub fn insertion() -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = insertion();

        assert_eq!(res, 42);
    }
}
