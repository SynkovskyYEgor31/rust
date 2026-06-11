#![allow(dead_code)]

pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;
    let max_a = *a.iter().max().unwrap_or(&1);
    let min_b = *b.iter().min().unwrap_or(&100);

    for x in max_a..=min_b {
        let is_factor_of_x = a.iter().all(|&element| x % element == 0);
        let x_is_factor_of_b = b.iter().all(|&element| element % x == 0);
        
        if is_factor_of_x && x_is_factor_of_b {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between_two_sets() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }
}

