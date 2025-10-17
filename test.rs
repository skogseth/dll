use libtest2::_private::{Case, DistributedList, DynCase};
use libtest2::{RunResult, TestContext};

static TESTS: DistributedList<DynCase> = DistributedList::root();

fn main() {
    for (i, test) in TESTS.iter().enumerate() {
        println!("[{i}] {n}", n = test.name());
    }

    libtest2::main(TESTS.iter().copied());
}

#[libtest2::test]
fn random_test(_context: &TestContext) -> RunResult {
    assert_eq!(1 + 1, 2);
    Ok(())
}

#[libtest2::test]
fn test_number_2(_context: &TestContext) -> RunResult {
    Ok(())
}

#[libtest2::test]
fn failing_test(_context: &TestContext) -> RunResult {
    panic!("shit we failed boys");
}
