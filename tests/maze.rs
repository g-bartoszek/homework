use homework::maze::{read_from_file, solve};

#[test]
fn test_maze() {
    assert_eq!(
        solve(read_from_file("tests/rsc/maze.in").unwrap()).unwrap(),
        4
    );
}
