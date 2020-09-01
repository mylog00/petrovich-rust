pub mod case;
pub mod gender;
mod rules;
use case::Case;
use gender::Gender;
use rules::{Rule, RuleGroup, Rules};
use std::error::Error;
use std::fmt;
use std::fs;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub struct Petrovich {
    rules: Yaml,
}

impl Petrovich {
    pub fn new(file_path: &str) -> Result<Petrovich, Box<dyn Error>> {
        let rules = load_rules(file_path)?;
        Result::Ok(Petrovich { rules })
    }

    pub fn first_name(
        &self,
        first_name: &str,
        gender: Gender,
        case: Case,
    ) -> Result<String, Box<dyn Error>> {
        print!("{} {} {}", first_name, gender, case);

        Result::Err(
            PetrovichError {
                message: String::from("Not implemented"),
            }
            .into(),
        )
    }
    pub fn last_name(
        &self,
        last_name: &str,
        gender: Gender,
        case: Case,
    ) -> Result<String, Box<dyn Error>> {
        print!("{} {} {}", last_name, gender, case);
        Result::Err(
            PetrovichError {
                message: String::from("Not implemented"),
            }
            .into(),
        )
    }
    pub fn middle_name(
        &self,
        middle_name: &str,
        gender: Gender,
        case: Case,
    ) -> Result<String, Box<dyn Error>> {
        print!("{} {} {}", middle_name, gender, case);
        Result::Err(
            PetrovichError {
                message: String::from("Not implemented"),
            }
            .into(),
        )
    }
}
fn inflect(rules: &Yaml, name: &str, case: Case, gender: Gender) -> String {
    let names: Vec<&str> = name.split('-').collect();
    return String::from("Test");
}

fn find_rule(rules: &Yaml, name: &str, gender: Gender, first_word: bool) {
    &rules["exceptions"].is_badvalue();
}

fn match_rule<'a>(
    rules: &'a Yaml,
    name: &str,
    gender: Gender,
    first_word: bool,
) -> Option<&'a Yaml> {
    if !gender.equal(rules["gender"].as_str().unwrap()) {
        return None;
    };
    if first_word && rules["tags"].is_badvalue() {
        return None;
    }
    if !first_word && !rules["tags"].is_badvalue() {
        return None;
    }
    for val in rules["test"].as_vec().unwrap() {
        if name.ends_with(val.as_str().unwrap()) {
            return Some(&rules["mods"]);
        }
    }
    None
}

fn find_postfix(mods: &Yaml, case: Case) -> Option<&str> {
    match case {
        Case::Nominative => Option::None,
        Case::Genitive => mods[0].as_str(),
        Case::Dative => mods[1].as_str(),
        Case::Accusative => mods[2].as_str(),
        Case::Instrumental => mods[3].as_str(),
        Case::Prepositional => mods[4].as_str(),
    }
}

fn apply_rule(name: &str, rule: &str) -> String {
    let mut name = String::from(name);
    let mut postfix = String::new();
    rule.chars().for_each(|c| match c {
        '-' => {
            name.pop();
        }
        '.' => {}
        _ => postfix.push(c),
    });
    name.push_str(&postfix);
    name
}

fn load_rules(filename: &str) -> Result<Yaml, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;

    YamlLoader::load_from_str(&content)?
        .get(0)
        .cloned()
        .ok_or_else(|| {
            PetrovichError {
                message: format!("Can't load rules from file '{}'", filename),
            }
            .into()
        })
}

#[derive(Debug)]
pub struct PetrovichError {
    message: String,
}

impl fmt::Display for PetrovichError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl Error for PetrovichError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_rule_test() {
        let rules = test_rules();

        let rule = &rules["firstname"]["exceptions"][0];
        let actual = match_rule(rule, "лев", Gender::Male, false);
        assert_eq!(Some(&rule["mods"]), actual);

        let rule = &rules["middlename"]["exceptions"][0];
        let actual = match_rule(rule, "борух", Gender::Male, true);
        assert_eq!(Some(&rule["mods"]), actual);

        let rule = &rules["lastname"]["exceptions"][0];
        let actual = match_rule(rule, "абдул", Gender::Male, false);
        assert_eq!(None, actual);
    }

    #[test]
    fn find_postfix_test() {
        let mods = &test_rules()["firstname"]["exceptions"][0]["mods"];

        let actual = find_postfix(&mods, Case::Nominative);
        assert_eq!(None, actual);

        let actual = find_postfix(&mods, Case::Genitive);
        assert_eq!(Some("--ьва"), actual);

        let actual = find_postfix(&mods, Case::Dative);
        assert_eq!(Some("--ьву"), actual);

        let actual = find_postfix(&mods, Case::Accusative);
        assert_eq!(Some("--ьва"), actual);

        let actual = find_postfix(&mods, Case::Instrumental);
        assert_eq!(Some("--ьвом"), actual);

        let actual = find_postfix(&mods, Case::Prepositional);
        assert_eq!(Some("--ьве"), actual);
    }

    #[test]
    fn apply_rule_test() {
        let actual = apply_rule("Маша", "-и");
        assert_eq!("Маши", actual);

        let actual = apply_rule("пётр", "---етру");
        assert_eq!("петру", actual);

        let actual = apply_rule("Маша", ".");
        assert_eq!("Маша", actual);

        let actual = apply_rule("", "-и");
        assert_eq!("и", actual);
    }

    #[test]
    fn first_name() {
        let rules = test_rules();
        println!("{:?}", rules["firstname"]["exceptions"][0]);
        let val = inflect(&rules, "", Case::Dative, Gender::Male);
        println!("{}", val);
    }

    fn test_rules() -> Yaml {
        YamlLoader::load_from_str(
            "
firstname:
  exceptions:
    - gender: male
      test: [лев]
      mods: [--ьва, --ьву, --ьва, --ьвом, --ьве]
  suffixes:
    - gender: androgynous
      test: [е, ё, и, о, у, ы, э, ю]
      mods: [., ., ., ., .]
lastname:
  exceptions:
    - gender: androgynous
      test:
        - бонч
        - абдул
      mods: [., ., ., ., .]
      tags: [first_word]
  suffixes:
    - gender: female
      test: [б, в, г, д, ж, з]
      mods: [., ., ., ., .]
middlename:
  exceptions:
    - gender: androgynous
      test:
        - борух
      mods: [., ., ., ., .]
      tags: [first_word]
  suffixes:
    - gender: male
      test: [мич, ьич, кич]
      mods: [а, у, а, ом, е]
",
        )
        .unwrap()
        .get(0)
        .unwrap()
        .clone()
    }
}
