use libtest2::_private::{Case, DistributedList, DynCase};
use libtest2::{RunResult, TestContext};

static TESTS: DistributedList<DynCase> = DistributedList::root();
static DIFFERENT_TESTS: DistributedList<DynCase> = DistributedList::root();

fn main() {
    let mut all_tests = Vec::new();

    for (i, test) in TESTS.iter().copied().enumerate() {
        eprintln!("[{i}] {n}", n = test.name());
        all_tests.push(test);
    }

    for (i, test) in DIFFERENT_TESTS.iter().copied().enumerate() {
        eprintln!("<<different>> [{i}] {n}", n = test.name());
        all_tests.push(test);
    }

    libtest2::main(all_tests);
}

libtest2::_test_parse! {
    #[test]
    fn random_test(_context: &TestContext) -> RunResult {
        assert_eq!(1 + 1, 2);
        Ok(())
    }
}

libtest2::_test_parse! {
    #[test]
    fn test_number_2(_context: &TestContext) -> RunResult {
        Ok(())
    }
}

libtest2::_test_parse! {
    #[test]
    fn failing_test(_context: &TestContext) -> RunResult {
        panic!("shit we failed boys");
    }
}

libtest2::_test_parse! {
    #[test(DIFFERENT_TESTS)]
    fn not_like_the_other_tests(_context: &TestContext) -> RunResult {
        panic!("I'm special!");
    }
}
