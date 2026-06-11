#![allow(dead_code)]
pub fn staircase_logic(n: i32) -> String {
    let mut result = String::new();
    let n = n as usize;
    for i in 1..=n {
        let spaces = " ".repeat(n - i);
        let hashes = "#".repeat(i);
        result.push_str(&spaces);
        result.push_str(&hashes);
        if i < n {
            result.push('\n');
        }
    }
    result
}

pub fn staircase(n: i32) {
    println!("{}", staircase_logic(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_size_4() {
        let expected = "   #\n  ##\n ###\n####";
        assert_eq!(staircase_logic(4), expected);
    }

    #[test]
    fn test_staircase_size_1() {
        assert_eq!(staircase_logic(1), "#");
    }
}

