//! The GS1 checksum algorithm

fn int_digits(input: &str) -> Vec<u16> {
    input.chars().map(|d| d.to_digit(10).unwrap() as u16).collect()
}

/// Calculate a GS1 checksum digit.
///
/// # Example
/// ```
/// # use gs1::checksum::gs1_checksum;
/// let code = "0360843951968";
/// gs1_checksum(&code.to_string());
/// ```
///
/// # Further Information
/// GS1 General Specifications Section 7.9.1 - a description can also be found [on the GS1
/// website](https://www.gs1.org/services/how-calculate-check-digit-manually).
pub fn gs1_checksum(input: &str) -> u8 {
    let digits = int_digits(input);
    let mut even: u16 = 0;
    let mut odd: u16 = 0;

    for i in 1..=digits.len() {
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
