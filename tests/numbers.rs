use homework::numbers::{convert_numbers, read_from_file};
use num_bigint::BigInt;

#[test]
fn test_numbers() {
    let result = convert_numbers(read_from_file("tests/rsc/maze.in").unwrap());

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
