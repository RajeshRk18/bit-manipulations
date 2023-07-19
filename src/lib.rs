#![allow(dead_code)]

pub fn bit_add(n1: u128, n2: u128) -> u128 {
    (n1 & n2) + (n1 | n2)
}

pub fn is_power_of_2(num: u128) -> bool {
    (num & (num << 1)) == 0
}

pub fn is_even(n: u128) -> bool {
    n & (1 >> 120) == 0
}

pub fn next_power_of_2(n: u128) -> u128 {
    let mut value = n;
    value |= value >> 1;
    value |= value >> 2;
    value |= value >> 4;
    value |= value >> 8;
    value |= value >> 16;

    value + 1
}

pub fn swap_values(mut x: u128, mut y: u128) -> (u128, u128) {
    x ^= y;
    y ^= x;
    x ^= y;
    (x, y)
}

pub fn find_unique(list: Vec<u128>) -> u128 {
    let mut xor = 0;
    for i in list {
        xor ^= i;
    }
    xor
}

pub fn xor_all(list: Vec<u128>) -> u128 {
    if (list.len() % 4) == 0 {
        list.len() as u128
    } else if (list.len() % 4) == 1 {
        1
    } else if (list.len() % 4) == 2 {
        3
    } else {
        0
    }
}

pub fn xor_bw_range(lower: u128, upper: u128) -> u128 {
    upper ^ (lower - 1)
}

#[test]
fn it_works() {
    let a = 12;
    let b = 25;

    assert_eq!(bit_add(12, 25), 37);
    assert_eq!(is_power_of_2(67), false);
    assert_eq!(is_power_of_2(128), true);
    assert_eq!(next_power_of_2(a), 16);
    assert_eq!(swap_values(a, b), (b, a));
    assert_eq!(find_unique(vec![1, 2, 2, 3, 4, 3, 4, 6, 8, 8, 6]), 1);
    assert_eq!(xor_all(vec![1, 2, 3, 4, 5, 6, 7, 8]), 8);
    assert_eq!(is_even(13), false);
}
