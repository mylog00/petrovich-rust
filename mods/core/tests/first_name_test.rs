mod common;

#[test]
fn first_name_misc_test() {
    let petrovich = common::load_petrovich();
    let test_cases = common::load_name_test_cases("firstnames.misc.tsv");
    for test in test_cases {
        let actual = petrovich.first_name(&test.name, &test.gender, &test.case);
        common::examine_answer(actual, &test.expected)
    }
}

#[ignore]
#[test]
fn first_name_test() {
    let petrovich = common::load_petrovich();
    let test_cases = common::load_name_test_cases("firstnames.tsv");
    for test in test_cases {
        let actual = petrovich.first_name(&test.name, &test.gender, &test.case);
        common::examine_answer(actual, &test.expected)
    }
}
