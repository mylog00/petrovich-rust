pub mod case;
pub mod errors;
pub mod gender;
mod rules;
use case::Case;
use gender::Gender;
use rules::{RuleGroup, Rules};
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

    pub fn first_name(&self, first_name: &str, gender: &Gender, case: &Case) -> String {
        inflect(&self.rules.first_name(), first_name, case, gender)
    }

    pub fn last_name(&self, last_name: &str, gender: &Gender, case: &Case) -> String {
        inflect(&self.rules.last_name(), last_name, case, gender)
    }

    pub fn middle_name(&self, middle_name: &str, gender: &Gender, case: &Case) -> String {
        inflect(&self.rules.middle_name(), middle_name, case, gender)
    }
}

fn inflect(rule_group: &RuleGroup, name: &str, case: &Case, gender: &Gender) -> String {
    let name = name.trim().to_lowercase();
    let name_vec: Vec<&str> = name.split('-').collect();
    let mut result: Vec<String> = Vec::with_capacity(name_vec.len());
    let mut is_first = true;
    for word in name_vec {
        let rule = rule_group.get_rule(word, gender, is_first);
        match rule {
            None => result.push(word.to_string()),
            Some(rule) => result.push(rule.apply(word, case)),
        }
        if is_first {
            is_first = false;
        }
    }
    result.join("-")
}
