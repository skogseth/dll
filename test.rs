use libtest2::_private::{Case, DistributedList, DynCase};
use libtest2::{RunResult, TestContext};

static TESTS: DistributedList<DynCase<i32>> = DistributedList::root();

fn main() {
    for (i, test) in TESTS.iter().enumerate() {
        println!("[{i}] {n}", n = test.name());
    }

    libtest2::main(TESTS.iter().copied(), 10);
}

libtest2::_private::test_parse! {
    #[test i32]
    fn random_test(_context: &TestContext<i32>) -> RunResult {
        assert_eq!(1 + 1, 2);
        Ok(())
    }
}

libtest2::_private::test_parse! {
    #[test i32]
    fn test_number_2(_context: &TestContext<i32>) -> RunResult {
        Ok(())
    }
}

libtest2::_private::test_parse! {
    #[test i32]
    fn failing_test(_context: &TestContext<i32>) -> RunResult {
        panic!("shit we failed boys");
    }
}
