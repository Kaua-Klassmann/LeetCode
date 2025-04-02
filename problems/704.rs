pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut l, mut r) = (0, nums.len() as i32 - 1);

    while l <= r {
        let mid = l + (r - l) / 2;

        if target == nums[mid as usize] {
            return mid;
        }

        if target > nums[mid as usize] {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    -1
}

fn main() {
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
}
