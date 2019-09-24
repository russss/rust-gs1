
fn int_digits(input: &String) -> Vec<u16> {
    input.chars().map(|d| d.to_digit(10).unwrap() as u16).collect()
}

pub fn gs1_checksum(input: &String) -> u8 {
    let digits = int_digits(input);
    let mut even: u16 = 0;
    let mut odd: u16 = 0;

    for i in 1..digits.len() + 1 {
        let curr = digits[digits.len() - i];
        if i % 2 == 0 {
            even += curr;
        } else {
            odd += curr;
        }
    }

    let mut check = (3 * odd + even) % 10;
    if check > 0 {
        check = 10 - check;
    }

    check as u8
}

#[test]
fn test_gs1_checksum() {
    assert_eq!(0, gs1_checksum(&"0360843951968".to_string()));
    assert_eq!(8, gs1_checksum(&"8061414112345".to_string()));
}
