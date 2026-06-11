#![allow(dead_code)]

pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut rounded_grades = Vec::new();
    for &grade in grades {
        if grade < 38 {
            rounded_grades.push(grade);
        } else {
            let next_multiple = ((grade / 5) + 1) * 5;
            if next_multiple - grade < 3 {
                rounded_grades.push(next_multiple);
            } else {
                rounded_grades.push(grade);
            }
        }
    }
    rounded_grades
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading() {
        let input = vec![73, 67, 38, 33];
        let expected = vec![75, 67, 40, 33];
        assert_eq!(grading_students(&input), expected);
    }
}

