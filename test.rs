use libtest2::_private::{DistributedList, TestDef};
use libtest2::{FnCase, RunResult, TestContext};

static TESTS: DistributedList<TestDef> = DistributedList::root();
static FIXTURE_TESTS: DistributedList<TestDef<fn(i32)>> = DistributedList::root();

fn main() {
    let basic_tests = TESTS
        .iter()
        .copied()
        .map(|test| FnCase::test(test.name, test.function));

    // Do some fake setup for the "fixture" tests
    let i = 30;

    let fixture_tests = FIXTURE_TESTS.iter().copied().map(|test| {
        FnCase::test(test.name, move |_ctx: &TestContext| -> RunResult {
            (test.function)(i);
            Ok(())
        })
    });

    let all_tests = basic_tests.chain(fixture_tests);
    libtest2::main(all_tests);
}

libtest2::_test_parse! {
    #[test(FIXTURE_TESTS, fn(i32))]
    fn fixture_test_1(i: i32) {
        assert!(i > 0);
    }
}

libtest2::_test_parse! {
    #[test(FIXTURE_TESTS, fn(i32))]
    fn fixture_test_2(i: i32) {
        assert!(i == 1);
    }
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
