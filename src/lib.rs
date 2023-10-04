
pub fn one_max(bits: &[u8]) -> i32 {
    let mut one_count: i32 = 0;
    for bit in bits.iter() {
        one_count += *bit as i32;
    }
    one_count
}

fn bit_to_sign(bit: u8) -> i32 {
    (bit as i32 * 2) - 1
}

pub fn labs(bits: &[u8]) -> i32 {
    let mut es: i32 = 0;
    for k in 1..bits.len() {
        let mut ck_s = 0;
        for i in 0..bits.len()-k {
            ck_s += bit_to_sign(bits[i]) * bit_to_sign(bits[i + k])
        }
        es += ck_s * ck_s;
    }
    es
}

#[cfg(test)]
mod tests {

    use super::*;

    fn check_one_max(bits: &[u8], res: i32) {
        assert_eq!(one_max(bits), res);
    }

    fn check_labs(bits: &[u8], res: i32) {
        assert_eq!(labs(bits), res);
    }

    #[test]
    fn tests_one_max() {
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

    #[test]
    fn tests_labs() {
        check_labs(&[0], 0);
        check_labs(&[1], 0);
        check_labs(&[0, 0], 1);
        check_labs(&[0, 1], 1);
        check_labs(&[1, 0], 1);
        check_labs(&[1, 1], 1);
        check_labs(&[0, 0, 0], 5);
        check_labs(&[0, 0, 1], 1);
        check_labs(&[0, 1, 0], 5);
        check_labs(&[0, 1, 1], 1);
        check_labs(&[1, 0, 0], 1);
        check_labs(&[1, 0, 1], 5);
        check_labs(&[1, 1, 0], 1);
        check_labs(&[1, 1, 1], 5);
        check_labs(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 285);
        check_labs(&[0, 1, 0, 1, 0, 1, 0, 1, 0, 1], 285);
        check_labs(&[0, 0, 1, 1, 0, 0, 1, 1, 0, 0], 125);
        check_labs(&[1, 1, 0, 0, 1, 1, 0, 0, 1, 1], 125);
        check_labs(&[0, 0, 0, 0, 0, 1, 1, 1, 1, 1], 125);
        check_labs(&[1, 1, 1, 1, 1, 0, 0, 0, 0, 0], 125);
        check_labs(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 285);
    }

}