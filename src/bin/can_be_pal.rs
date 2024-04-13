// This problem was asked by Google.
// Given a string which we can delete at most k, return whether you can make a palindrome.
// For example, given 'waterrfetawx' and a k of 2, you could delete f and x to get 'waterretaw'.
// 

use std::collections::VecDeque;
use itertools::Itertools;

fn can_be_pal(word: String, mut rem:  i32) -> bool {
    print!("Another word! {word}\n");
    let mut word_fwd = VecDeque::from_iter(word.chars());
    let mut word_rev = VecDeque::from_iter(word.chars().rev());
    while rem > 0  {
        println!("FWD: {word_fwd:?}");
        println!("REV: {word_rev:?}");
        if word_fwd[0] == word_rev[0] {
            word_fwd.pop_front();
            word_rev.pop_front();
        }
        else {
            rem -= 1;
            let removed = word_rev.pop_front().unwrap_or_default();
            for i in 0..word_fwd.len() {
                if removed == word_fwd[i] {
                    word_fwd.remove(i);
                    break
                }
            }
        }
        if  word_rev.iter().join("") == word_fwd.iter().join(""){
            return true
        }
    }
    false
}


fn main () {
    assert!(can_be_pal(String::from("waterrfetawx"), 2)); //true
    assert!(!can_be_pal(String::from("waterrfetawx"), 1)); //false
    assert!(!can_be_pal(String::from("definitelynotapal"), 2));//false
    assert!(!can_be_pal(String::from("definitelynotapal"), 15)); // false
    // Im aware it breaks if K >= String.len() 
}