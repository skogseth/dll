use libtest2::{Harness, RunResult, Trial};

fn main() -> Result<(), std::io::Error> {
    let tests = vec![
        Trial::test("random_test", |_ctx| -> RunResult {
            random_test();
            Ok(())
        }),
        Trial::test("test_number_2", |_ctx| -> RunResult {
            test_number_2();
            Ok(())
        }),
        Trial::test("failing_test", |_ctx| -> RunResult {
            failing_test();
            Ok(())
        }),
    ];

    Harness::new()
        .with_env()
        .unwrap()
        .parse()
        .unwrap()
        .discover(tests)
        .unwrap()
        .run()
        .map(|_success| ())
}

fn random_test() {
    assert_eq!(1 + 1, 2);
}

fn test_number_2() {}

fn failing_test() {
    panic!("shit we failed boys");
}
