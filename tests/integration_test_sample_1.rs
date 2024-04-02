#![allow(dead_code)] 
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_attributes)]
use std::cell::RefCell;

use test_rust::rust_lang::borrow_mut;
use test_rust::rust_lang::trait_sample;

#[macro_use]
use test_rust::log_a;

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}
impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}
impl borrow_mut::Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

#[test]
fn test_111() {
    let x = 5;
    log_a!("tests excute...");
    assert_eq!(5, x);
}

#[test]
fn test_trait_sample() {
    trait_sample::do_area();
}

#[test]
fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = borrow_mut::LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
}
