
use break_block_macro::bb;

#[test]
fn simple_result_test() {

    let result = 'block: {
        let one = bb!(Ok("one"));
        assert_eq!(one, "one");

        let _two = bb!(Err("two"));
        Ok("three")
    };

    assert_eq!(result, Err("two"));
}

#[test]
fn simple_option_test() {

    let result = 'block: {
        let one = bb!(Some("one"));
        assert_eq!(one, "one");

        let _two = bb!(None);
        Some("three")
    };

    assert_eq!(result, None);
}
