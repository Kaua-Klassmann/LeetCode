pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    if x < 10 {
        return true;
    }

    let x_string = x.to_string();

    let (mut l, mut r) = (0, x_string.len() - 1);

    while l <= r {
        if x_string.as_bytes()[l] != x_string.as_bytes()[r] {
            return false;
        }

        l += 1;
        r -= 1;
    }

    true
}

fn main() {
    assert_eq!(true, is_palindrome(121));
    assert_eq!(false, is_palindrome(-121));
    assert_eq!(false, is_palindrome(10))
}
