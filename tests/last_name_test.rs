mod common;

#[test]
fn last_name_misc_test() {
    let petrovich = common::load_petrovich();
    let test_cases = common::load_name_test_cases("surnames.misc.tsv");
    for test in test_cases {
        let actual = petrovich.last_name(&test.name, &test.gender, &test.case);
        common::examine_answer(actual, &test.expected)
    }
}

#[ignore]
#[test]
fn last_name_test() {
    let petrovich = common::load_petrovich();
    let test_cases = common::load_name_test_cases("surnames.tsv");
    for test in test_cases {
        let actual = petrovich.last_name(&test.name, &test.gender, &test.case);
        common::examine_answer(actual, &test.expected)
    }
}
