use std::collections;
use std::i32;

pub struct Logger {
    history: collections::HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Logger {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Logger {
            history: collections::HashMap::new(),
        }
    }

    /** Returns true if the message should be printed in the given timestamp, otherwise returns false.
    If this method returns false, the message will not be printed.
    The timestamp is in seconds granularity. */
    pub fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        let last_printed = self.history.get(&message).unwrap_or(&i32::MIN);
        if *last_printed > timestamp - 10 {
            false
        } else {
            self.history.insert(message, timestamp);
            true
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger::new()
    }
}

/**
 * Your Logger object will be instantiated and called as such:
 * let obj = Logger::new();
 * let ret_1: bool = obj.should_print_message(timestamp, message);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_validate_example1() {
        // GIVEN
        let mut logger = Logger::new();

        // WHEN
        let print_1 = logger.should_print_message(1, "foo".to_string());
        let print_2 = logger.should_print_message(2, "bar".to_string());
        let print_3 = logger.should_print_message(3, "foo".to_string());
        let print_8 = logger.should_print_message(8, "bar".to_string());
        let print_10 = logger.should_print_message(10, "foo".to_string());
        let print_11 = logger.should_print_message(11, "foo".to_string());

        // THEN
        assert_eq!(print_1, true);
        assert_eq!(print_2, true);
        assert_eq!(print_3, false);
        assert_eq!(print_8, false);
        assert_eq!(print_10, false);
        assert_eq!(print_11, true);
    }
}
