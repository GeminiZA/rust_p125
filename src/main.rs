struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect();
        for i in 0..chars.len()/2 {
            if chars[i] != chars[chars.len() - i - 1] {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let i: String = " ".to_string();
    let result = Solution::is_palindrome(i);
    println!("{:?}", result);
}
