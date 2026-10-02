use std::io::{self, Read};

fn main() {
    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .expect("failed to read input");

    let text = text.trim_end_matches(|character| character == '\n' || character == '\r');
    println!("{}", longest_palindromic_substring(text));
}

fn longest_palindromic_substring(_text: &str) -> &str {
    let b = _text.as_bytes();
    let n = b.len();
    if n == 0 {
        return "";
    }
    let (mut start, mut best) = (0, 1);

    for i in 0..n {
        for (mut l, mut r) in [(i as isize, i as isize), (i as isize, i as isize + 1)] {
            while l >= 0 && (r as usize) < n && b[l as usize] == b[r as usize] {
                l -= 1;
                r += 1;
            }
            let len = (r - l - 1) as usize;
            if len > best {
                best = len;
                start = (l + 1) as usize;
            }
        }
    }
    &_text[start..start + best]
}

#[cfg(test)]
#[path = "../tests/unit/palindrome.rs"]
mod palindrome_tests;
