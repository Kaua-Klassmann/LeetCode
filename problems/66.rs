pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    *digits.last_mut().unwrap() += 1;

    for index in (0..digits.len()).rev() {
        if digits[index] < 10 {
            break;
        }

        digits[index] = 0;

        if index == 0 {
            digits.insert(0, 1);
        } else {
            digits[index - 1] += 1;
        }
    }

    digits
}

fn main() {
    assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    assert_eq!(plus_one(vec![9]), vec![1, 0]);
}
