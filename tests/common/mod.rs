use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use petrovich::case::Case;
use petrovich::gender::Gender;
use petrovich::Petrovich;

const RULES_FILE: &str = "petrovich-rules/rules.yml";
const TEST_FILE_DIR: &str = "petrovich-eval/";

pub fn load_petrovich() -> Petrovich {
    Petrovich::new(RULES_FILE).unwrap_or_else(|error| panic!("Problem reading rules: {:?}", error))
}

pub fn examine_answer(actual: String, expected: &str) {
    assert_eq!(
        actual, expected,
        "Actual value:'{}' Expected value:'{}'",
        actual, expected
    )
}

pub struct NameTestCase {
    pub name: String,
    pub expected: String,
    pub gender: Gender,
    pub case: Case,
}

pub fn load_name_test_cases(file_name: &str) -> Vec<NameTestCase> {
    read_lines(format!("{}{}", TEST_FILE_DIR, file_name))
        .skip(1)
        .map(|line| map_2_name_test_case(&line.unwrap()))
        .collect()
}

fn map_2_name_test_case(line: &str) -> NameTestCase {
    let split: Vec<&str> = line.split('\t').collect();
    let name = split.get(0).unwrap().to_string();
    let expected = split.get(1).unwrap().to_lowercase();
    let split: Vec<&str> = split.get(2).unwrap().split(',').collect();
    let gender = get_gender(split.get(0).unwrap());
    let case = get_case(split.get(2).unwrap());
    NameTestCase {
        name,
        expected,
        gender,
        case,
    }
}

fn get_gender(gender: &str) -> Gender {
    match gender {
        "мр" => Gender::Male,
        "жр" => Gender::Female,
        _ => Gender::Androgynous,
    }
}

fn get_case(gender: &str) -> Case {
    match gender {
        "рд" => Case::Genitive,
        "дт" => Case::Dative,
        "вн" => Case::Accusative,
        "тв" => Case::Instrumental,
        "пр" => Case::Prepositional,
        _ => Case::Nominative,
    }
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
