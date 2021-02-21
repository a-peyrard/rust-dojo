/*
    En Route Salute

    Foobar problem weirdly solved a year ago, here:
    https://github.com/a-peyrard/foobar/blob/master/src/main/java/org/teutinc/foobar/second/part1/Solution.java
 */

pub fn count_salute(world: &str) -> i32 {
    let mut right_walkers = 0;
    let mut salutes = 0;
    for char in world.chars() {
        match char {
            '>' => right_walkers += 1,
            '<' => salutes += right_walkers,
            _ => (),
        }
    }
    salutes * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_should_validate_first_case() {
        // GIVEN
        let s = "<<>><";

        // WHEN
        let res = count_salute(s);

        // THEN
        assert_eq!(res, 4);
    }

    #[test]
    fn it_should_should_validate_second_case() {
        // GIVEN
        let s = ">----<";

        // WHEN
        let res = count_salute(s);

        // THEN
        assert_eq!(res, 2);
    }

    #[test]
    fn it_should_should_validate_no_crossing_case() {
        // GIVEN
        let s = "--<<--->>>>---";

        // WHEN
        let res = count_salute(s);

        // THEN
        assert_eq!(res, 0);
    }

    #[test]
    fn it_should_should_validate_no_employees_case() {
        // GIVEN
        let s = "--------";

        // WHEN
        let res = count_salute(s);

        // THEN
        assert_eq!(res, 0);
    }

    #[test]
    fn it_should_should_validate_not_everyone_crossing_each_other_case() {
        // GIVEN
        let s = ">><>><";

        // WHEN
        let res = count_salute(s);

        // THEN
        assert_eq!(res, 12);
    }
}
