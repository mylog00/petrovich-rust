use petrovich::case::Case;
use petrovich::gender::Gender;
use petrovich::Petrovich;

const RULES_FILE: &str = "petrovich-rules/rules.yml";

#[test]
fn examine_last_name() {
    let petrovich = load_petrovich();
    let last_name = "дубовицкая";
    let gender = Gender::Female;

    let actual = petrovich.last_name(last_name, &gender, &Case::Nominative);
    examine_answer(actual, "дубовицкая");

    let actual = petrovich.last_name(last_name, &gender, &Case::Genitive);
    examine_answer(actual, "дубовицкой");

    let actual = petrovich.last_name(last_name, &gender, &Case::Dative);
    examine_answer(actual, "дубовицкой");

    let actual = petrovich.last_name(last_name, &gender, &Case::Accusative);
    examine_answer(actual, "дубовицкую");

    let actual = petrovich.last_name(last_name, &gender, &Case::Instrumental);
    examine_answer(actual, "дубовицкой");

    let actual = petrovich.last_name(last_name, &gender, &Case::Prepositional);
    examine_answer(actual, "дубовицкой");
}

#[test]
fn examine_first_name() {
    let petrovich = load_petrovich();
    let first_name = "пётр";
      let gender = Gender::Male;
    
    let actual = petrovich.first_name(first_name, &gender, &Case::Nominative);
    examine_answer(actual, "пётр");

    let actual = petrovich.first_name(first_name, &gender, &Case::Genitive);
    examine_answer(actual, "петра");

    let actual = petrovich.first_name(first_name, &gender, &Case::Dative);
    examine_answer(actual, "петру");

    let actual = petrovich.first_name(first_name, &gender, &Case::Accusative);
    examine_answer(actual, "петра");

    let actual = petrovich.first_name(first_name, &gender, &Case::Instrumental);
    examine_answer(actual, "петром");

    let actual = petrovich.first_name(first_name, &gender, &Case::Prepositional);
    examine_answer(actual, "петре");
}

fn examine_answer(actual: String, expected: &str) {
    assert_eq!(
        actual, expected,
        "Actual value:'{}' Expected value:'{}'",
        actual, expected
    )
}

fn load_petrovich() -> Petrovich {
    Petrovich::new(RULES_FILE).unwrap_or_else(|error| panic!("Problem reading rules: {:?}", error))
}
