pub mod case;
pub mod errors;
pub mod gender;
mod rules;
use case::Case;
use errors::PetrovichError;
use gender::Gender;
use rules::{Rule, RuleGroup, Rules};
use std::error::Error;

#[derive(Debug)]
pub struct Petrovich {
    rules: Rules,
}

impl Petrovich {
    pub fn new(file_path: &str) -> Result<Petrovich, Box<dyn Error>> {
        let rules = Rules::load_from_file(file_path)?;
        Result::Ok(Petrovich { rules })
    }

    pub fn first_name(&self, first_name: &str, gender: Gender, case: Case) -> String {
        inflect(&self.rules.first_name(), first_name, case, gender)
    }
    pub fn last_name(
        &self,
        last_name: &str,
        gender: Gender,
        case: Case,
    ) -> String {
        inflect(&self.rules.last_name(), last_name, case, gender)
    }
    pub fn middle_name(
        &self,
        middle_name: &str,
        gender: Gender,
        case: Case,
    ) -> String {
        inflect(&self.rules.middle_name(), middle_name, case, gender)
    }
}
fn inflect(rules: &RuleGroup, name: &str, case: Case, gender: Gender) -> String {
    let names: Vec<&str> = name.split('-').collect();
    return String::from("Test");
}

fn find_rule(rules: &RuleGroup, name: &str, gender: Gender, first_word: bool) {}

fn match_rule<'a>(
    rule: &'a Rule,
    name: &str,
    gender: Gender,
    first_word: bool,
) -> Option<&'a Vec<String>> {
    if !gender.equal(rule.gender()) {
        return None;
    };

    //if it is a first word then tags must present
    //if it is NOT a first word then tags must absent
    //in other cases return 'None'
    if first_word != rule.has_tags() {
        return None;
    }
    for val in rule.test() {
        if name.ends_with(val) {
            return Some(&rule.mods());
        }
    }
    None
}

fn find_postfix<'a>(mods: &'a Vec<String>, case: Case) -> Option<&'a String> {
    match case {
        Case::Nominative => Option::None,
        Case::Genitive => mods.get(0),
        Case::Dative => mods.get(1),
        Case::Accusative => mods.get(2),
        Case::Instrumental => mods.get(3),
        Case::Prepositional => mods.get(4),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_rule_test() {
        let rules = &test_rules();

        let rule = rules.first_name().exceptions().get(0).unwrap();
        let actual = match_rule(rule, "лев", Gender::Male, false);
        assert_eq!(Some(rule.mods()), actual);

        let rule = &rules.middle_name().exceptions().get(0).unwrap();
        let actual = match_rule(rule, "борух", Gender::Male, true);
        assert_eq!(Some(rule.mods()), actual);

        let rule = &rules.last_name().exceptions().get(0).unwrap();
        let actual = match_rule(rule, "абдул", Gender::Male, false);
        assert_eq!(None, actual);
    }

    #[test]
    fn find_postfix_test() {
        let rules = test_rules();
        let mods = rules.first_name().exceptions().get(0).unwrap().mods();

        let actual = find_postfix(&mods, Case::Nominative);
        assert_eq!(None, actual);

        let actual = find_postfix(&mods, Case::Genitive);
        assert_eq!(Some(&"--ьва".to_string()), actual);

        let actual = find_postfix(&mods, Case::Dative);
        assert_eq!(Some(&"--ьву".to_string()), actual);

        let actual = find_postfix(&mods, Case::Accusative);
        assert_eq!(Some(&"--ьва".to_string()), actual);

        let actual = find_postfix(&mods, Case::Instrumental);
        assert_eq!(Some(&"--ьвом".to_string()), actual);

        let actual = find_postfix(&mods, Case::Prepositional);
        assert_eq!(Some(&"--ьве".to_string()), actual);
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

    fn test_rules() -> Rules {
        Rules::load_from_str(
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
    }
}
