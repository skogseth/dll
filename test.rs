use libtest2::{RunError, TestContext};

#[libtest2::main]
fn main() {}

#[libtest2::test]
fn test_number_one(_ctx: &TestContext) -> Result<(), RunError> {
    assert_eq!(1 + 1, 2);
    Ok(())
}
