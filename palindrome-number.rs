impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let orig: String = x.to_string();
        let reverse: String = orig.chars().rev().collect();

        // borrowing 
        if orig.eq(&reverse) {
            true
        } else {
            false
        }
    }
}
