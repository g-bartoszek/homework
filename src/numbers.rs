use std::{error::Error, fs::File, io::BufRead, io::BufReader, path::Path};

use num_bigint::BigInt;

/// Converts strings containing binary numbers into integers
///
/// The input values may be up to 10000 bits long, hence the use of BigInt
pub fn convert_numbers(input: impl IntoIterator<Item = String>) -> Vec<Result<BigInt, String>> {
    input
        .into_iter()
        .map(|s| {
            BigInt::parse_bytes(s.as_bytes(), 2).ok_or_else(|| format!("Failed to parse: {s}"))
        })
        .collect()
}

pub fn read_from_file(path: impl AsRef<Path>) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    buf.lines().skip(1).map(|l| l.map_err(Into::into)).collect()
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;

    use super::convert_numbers;

    #[test]
    fn test_example_from_task_description() {
        let input = [
            "000000000".into(),
            "111101110".into(),
            "010101010".into(),
            "010111010".into(),
            "010000010".into(),
            "011110010".into(),
            "010011111".into(),
            "000000000".into(),
        ];

        let result = convert_numbers(input);

        assert_eq!(
            result,
            vec![
                Ok(BigInt::from(0)),
                Ok(BigInt::from(494)),
                Ok(BigInt::from(170)),
                Ok(BigInt::from(186)),
                Ok(BigInt::from(130)),
                Ok(BigInt::from(242)),
                Ok(BigInt::from(159)),
                Ok(BigInt::from(0))
            ]
        );
    }

    #[test]
    fn test_big_int() {
        let input = [
            "1100001011001101001100110101011001111110110110011100101111011000110000001000011101111010110001001010110111110101010110100001000101001001101000100011001000111100111001100001101101010100101000111011101101011011100000100110000001111000001001101100001101000000001110101001011000011111000000111001001000000000000100011110001011110001001001010000111000100100001001100001011000010011011110010001010001000000010001010100011100101100000000011011100100001100011110011000111000010100011010000011110000100111100101001000110001101110101100100111000110100110010111101011011111100001111000111100001011001111100101010010010000000001110010101100010001100010101000010010111000000010100010001110110100010000010011000111100111000010011001100100100010011011111001011111001010110010000010110000000001001111101010101001010110101011001100010011011110111001011111011001000011011011101111111011100000101110001000110001011101101000000101000011100011100111010101100000011100011110001101110010100000101101111000011000110010".into(),
        ];

        let result = convert_numbers(input);

        assert_eq!(
            result,
            vec![
                Ok(BigInt::parse_bytes(b"127399629618101871061164714369415138889690993275437627420444094492320543272307283661358790533838050723166537248941870298278768240542968408532693894495600991462888770154271773743688518108469133360749226067433284253841524878382663213020300439013395051660095465213244114983915020095532593844192661702194", 10).unwrap()),
            ]
        );
    }

    #[test]
    fn test_invalid_input() {
        let input = ["113".into(), "ASDF".into()];

        let result = convert_numbers(input);

        assert!(result[0].is_err());
        assert!(result[1].is_err());
    }
}
