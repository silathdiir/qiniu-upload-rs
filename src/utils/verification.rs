extern crate flate2;

use flate2::Crc;

pub(crate) fn crc32(data: &[u8]) -> u32 {
    let mut crc = Crc::new();
    crc.update(data);

    crc.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32() {
        let result = crc32(&b"1234".to_vec());
        assert_eq!(result, 2615402659);
    }
}
