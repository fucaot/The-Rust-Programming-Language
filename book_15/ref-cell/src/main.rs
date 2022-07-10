use std::fmt::Display;

pub trait Messenger {
    fn send(&self, message: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger: messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota！");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You're used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("warning: You're used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, vec};
    use super::*;

    struct MockMessenger {
        sent_message: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_message: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_message.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_message = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_message, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_message.sent_message.borrow().len(), 1);
    }
}

fn main() {
    // 无法可变的借用一个不可变的引用
    // let x = 5;
    // let y = &mut x;
}