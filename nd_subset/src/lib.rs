pub fn non_divisible_subset(k: i32, s: &[i32]) -> i32 {
    let mut remainder_list= vec![0 as i32; k as usize];
    let mut result = 0;
    for i in s {
        remainder_list[(i % k) as usize] += 1;
    }

    let bound = {
        if k % 2 == 0 {
            if remainder_list[(k/2) as usize] > 0 {
                result += 1;
            }
            k / 2
        }
        else {
            k / 2 + 1
        }
    };
    for i in 1..bound {
        result += std::cmp::max(remainder_list[i as usize], remainder_list[(k-i) as usize]);
    }
    if remainder_list[0] > 0 {
        result += 1;
    }
    result
}

#[test]
fn test_case_1() {
    let s = vec![19, 10, 12, 10, 24, 25, 22];
    assert_eq!(non_divisible_subset(4, &s), 3);
}

#[test]
fn test_case_2() {
    let s = vec![1, 7, 2, 4];
    assert_eq!(non_divisible_subset(3, &s), 3);
}
