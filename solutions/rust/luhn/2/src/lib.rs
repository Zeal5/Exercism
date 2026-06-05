/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // todo!("Is the Luhn checksum for {code} valid?");
    let digit_count = code.chars().filter(|c| c.is_ascii_digit()).count();

    if digit_count <= 1 {
        return false;
    }


    if code.chars().any(|c| !c.is_ascii_digit() && c != ' ') {
        return false;
    }
    let code: u32 = code
        .chars()
        .filter(|x| x.is_ascii_digit())
        .rev()
        .zip([false, true].into_iter().cycle())
        .map(|(i, double)| {
            let n = i.to_digit(10).unwrap();

            if double {
                n * 2 - 9 * (n > 4) as u32
            } else {
                n
            }
        })
        .sum();
    code.is_multiple_of(10)

}

