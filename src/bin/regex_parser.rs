use std::collections::VecDeque;

// Implement regular expression matching with the following special characters:
// . (period) which matches any single character
// * (asterisk) which matches zero or more of the preceding element
// That is, implement a function that takes in a string and a valid regular expression 
// and returns whether or not the string matches the regular expression.
// For example, given the regular expression "ra." and the string "ray", 
// your function should return true. The same regular expression on the string "raymond" should return false.
// Given the regular expression ".*at" and the string "chat", 
// your function should return true. The same regular expression on the string "chats" should return false.
const ANY_CHAR: char = '.';
const ANY_NUM: char = '*';

fn is_match(regex: String, word: String) -> bool {
    let mut regex_stack: VecDeque<char> = regex.chars().collect();
    let mut word_stack: VecDeque<char> = word.chars().collect();
    let mut matched = '-';
    //finite state machine
    while  regex_stack.len() > 0 {
        println!("regex: {:?} word: {:?}", regex_stack, word_stack);
        if regex_stack[0] == ANY_CHAR  || regex_stack[0] == word_stack[0] {
            matched = regex_stack.pop_front().unwrap();
            word_stack.pop_front();
        }
        else if regex_stack[0] == ANY_NUM && regex_stack[1] == word_stack[0] {
            regex_stack.pop_front();
        }
        else if regex_stack[0] ==  ANY_NUM && (matched == ANY_CHAR || matched == word_stack[0]){
            word_stack.pop_front();
        }
    }
    word_stack.len() == 0
}

fn main() {
    assert!(is_match("ra.".to_string(), "ray".to_string()));
    assert!(!is_match("ra.".to_string(), "raymond".to_string()));
    assert!(is_match(".*at".to_string(), "chat".to_string()));
    assert!(!is_match(".*at".to_string(), "chats".to_string()));
    assert!(is_match(".*peloton".to_string(), "onepeloton".to_string()))
}