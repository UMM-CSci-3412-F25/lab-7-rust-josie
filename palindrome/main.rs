use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args .len() - 1 == 1  {
        let input = &args[1];
        println!("Input: \"{}\"", input);
        println!("Is palindrome: {}", palindrome(input));
    } else {
        panic!("Please provide a single string argument to check for palindrome, you provided {} arguments.", args.len() - 1);
    }
}

fn palindrome(s: &str) -> String {
    let reversed = str_reverse(s);
    if s == reversed {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

fn str_reverse(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_reverse_simple() {
        assert_eq!(str_reverse("hello"), "olleh");
    }

    #[test]
    fn test_str_reverse_empty() {
        assert_eq!(str_reverse(""), "");
    }

    #[test]
    fn test_str_reverse_single_char() {
        assert_eq!(str_reverse("a"), "a");
    }

    #[test]
    fn test_str_reverse_palindrome() {
        assert_eq!(str_reverse("aba"), "aba");
    }

    #[test]
    fn test_str_reverse_with_spaces() {
        assert_eq!(str_reverse("hello world"), "dlrow olleh");
    }

    #[test]
    fn test_str_reverse_with_punctuation() {
        assert_eq!(str_reverse("Hello, world!"), "!dlrow ,olleH");
    }

    #[test]
    fn test_palindrome_true_simple() {
        assert_eq!(palindrome("aba"), "Yes");
    }

    #[test]
    fn test_palindrome_true_even_length() {
        assert_eq!(palindrome("abba"), "Yes");
    }

    #[test]
    fn test_palindrome_false() {
        assert_eq!(palindrome("hello"), "No");
    }

    #[test]
    fn test_palindrome_single_char() {
        assert_eq!(palindrome("a"), "Yes");
    }

    #[test]
    fn test_palindrome_empty() {
        assert_eq!(palindrome(""), "Yes");
    }

    #[test]
    fn test_palindrome_case_sensitive() {
        assert_eq!(palindrome("Aba"), "No");
    }

    #[test]
    fn test_palindrome_with_spaces() {
        assert_eq!(palindrome("a b a"), "Yes");
        assert_eq!(palindrome("a ba"), "No");
    }

    #[test]
    fn test_palindrome_with_punctuation() {
        assert_eq!(palindrome("a!a"), "Yes");
        assert_eq!(palindrome("Hello, world!"), "No");
    }

    #[test]
    fn test_palindrome_numbers() {
        assert_eq!(palindrome("12321"), "Yes");
        assert_eq!(palindrome("12345"), "No");
    }

    #[test]
    fn test_str_reverse_unicode() {
        assert_eq!(str_reverse("café"), "éfac");
        assert_eq!(str_reverse("🚀🌟"), "🌟🚀");
    }

    #[test]
    fn test_palindrome_unicode() {
        assert_eq!(palindrome("🚀🌟🚀"), "Yes");
        assert_eq!(palindrome("🚀🚀"), "Yes");
    }
}
