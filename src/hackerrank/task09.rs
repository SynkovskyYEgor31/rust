#![allow(dead_code)]

pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = vec![0; 6];
    for &bird in arr {
        if bird >= 1 && bird <= 5 {
            counts[bird as usize] += 1;
        }
    }

    let mut max_count = 0;
    let mut bird_type = 1;

    for i in 1..=5 {
        if counts[i] > max_count {
            max_count = counts[i];
            bird_type = i as i32;
        }
    }

    bird_type
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }
}

