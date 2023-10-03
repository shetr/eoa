
pub fn one_max(bits: &[u8]) -> u32 {
    let mut one_count: u32 = 0;
    for bit in bits.iter() {
        one_count += *bit as u32;
    }
    one_count
}

#[cfg(test)]
mod tests {

    use super::*;

    fn check_one_max(bits: &[u8], res: u32) {
        assert_eq!(one_max(bits), res);
    }

    #[test]
    fn tests_one_max() {
        check_one_max(&[0], 0);
        check_one_max(&[0], 0);
        check_one_max(&[0], 0);
        check_one_max(&[1], 1);
        check_one_max(&[0, 0], 0);
        check_one_max(&[0, 1], 1);
        check_one_max(&[1, 0], 1);
        check_one_max(&[1, 1], 2);
        check_one_max(&[0, 0, 0], 0);
        check_one_max(&[0, 0, 1], 1);
        check_one_max(&[0, 1, 0], 1);
        check_one_max(&[0, 1, 1], 2);
        check_one_max(&[1, 0, 0], 1);
        check_one_max(&[1, 0, 1], 2);
        check_one_max(&[1, 1, 0], 2);
        check_one_max(&[1, 1, 1], 3);
        check_one_max(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0);
        check_one_max(&[0, 1, 0, 1, 0, 1, 0, 1, 0, 1], 5);
        check_one_max(&[0, 0, 1, 1, 0, 0, 1, 1, 0, 0], 4);
        check_one_max(&[1, 1, 0, 0, 1, 1, 0, 0, 1, 1], 6);
        check_one_max(&[0, 0, 0, 0, 0, 1, 1, 1, 1, 1], 5);
        check_one_max(&[1, 1, 1, 1, 1, 0, 0, 0, 0, 0], 5);
        check_one_max(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 10);
    }

}