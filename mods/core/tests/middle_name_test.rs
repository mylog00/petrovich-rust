mod common;

#[test]
fn middle_name_misc_test() {
    let petrovich = common::load_petrovich();
    let test_cases = common::load_name_test_cases("midnames.misc.tsv");
    for test in test_cases {
        let actual = petrovich.middle_name(&test.name, &test.gender, &test.case);
        common::examine_answer(actual, &test.expected)
    }
}

#[test]
fn middle_name_test() {
    let petrovich = common::load_petrovich();
    let test_cases = common::load_name_test_cases("midnames.tsv");
    for test in test_cases {
        let actual = petrovich.middle_name(&test.name, &test.gender, &test.case);
        common::examine_answer(actual, &test.expected)
    }
}
