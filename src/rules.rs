use crate::errors::PetrovichError;
use crate::gender::Gender;
use std::error::Error;
use std::fs;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub struct Rules {
    first_name: RuleGroup,
    last_name: RuleGroup,
    middle_name: RuleGroup,
}

impl Rules {
    pub fn first_name(&self) -> &RuleGroup {
        &self.first_name
    }
    pub fn last_name(&self) -> &RuleGroup {
        &self.last_name
    }
    pub fn middle_name(&self) -> &RuleGroup {
        &self.middle_name
    }
    pub fn load_from_file(filename: &str) -> Result<Rules, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        Rules::load_from_str(&content)
    }

    pub fn load_from_str(content: &str) -> Result<Rules, Box<dyn Error>> {
        let yaml = YamlLoader::load_from_str(&content)?;
        let yaml = yaml.get(0);
        match yaml {
            None => Result::Err(PetrovichError::new("Can't parse yaml rules").into()),
            Some(yaml) => Result::Ok(Rules::of(&yaml)),
        }
    }

    fn of(rules: &Yaml) -> Rules {
        let first_name = &rules["firstname"];
        let last_name = &rules["lastname"];
        let middle_name = &rules["middlename"];
        Rules {
            first_name: RuleGroup::of(&first_name),
            last_name: RuleGroup::of(&last_name),
            middle_name: RuleGroup::of(&middle_name),
        }
    }
}

#[derive(Debug)]
pub struct RuleGroup {
    exceptions: Vec<Rule>,
    suffixes: Vec<Rule>,
}

impl RuleGroup {
    pub fn exceptions(&self) -> &Vec<Rule> {
        &self.exceptions
    }
    pub fn suffixes(&self) -> &Vec<Rule> {
        &self.suffixes
    }
    fn of(rules: &Yaml) -> RuleGroup {
        let exceptions = to_rule_vec(&rules["exceptions"]);
        let suffixes = to_rule_vec(&rules["suffixes"]);
        RuleGroup {
            exceptions,
            suffixes,
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    gender: Gender,
    test: Vec<String>,
    mods: Vec<String>,
    tags: bool,
}

impl Rule {
    pub fn gender(&self) -> &Gender {
        &self.gender
    }
    pub fn test(&self) -> &Vec<String> {
        &self.test
    }
    pub fn mods(&self) -> &Vec<String> {
        &self.mods
    }
    pub fn has_tags(&self) -> bool {
        self.tags
    }

    fn of(rules: &Yaml) -> Option<Rule> {
        let gender = Gender::of(&rules["gender"].as_str()?)?;
        let test = to_str_vec(&rules["test"]);
        let mods = to_str_vec(&rules["mods"]);
        let tags = !rules["tags"].is_badvalue();

        let rule = Rule {
            gender,
            test,
            mods,
            tags,
        };
        Some(rule)
    }
}

fn to_str_vec(array: &Yaml) -> Vec<String> {
    if !array.is_array() {
        return Vec::new();
    }
    let mut result = Vec::new();
    for node in array.as_vec().unwrap() {
        if let Some(val) = node.as_str() {
            result.push(String::from(val));
        }
    }
    result
}

fn to_rule_vec(array: &Yaml) -> Vec<Rule> {
    if !array.is_array() {
        return Vec::new();
    }
    let mut result = Vec::new();
    for node in array.as_vec().unwrap() {
        if let Some(rule) = Rule::of(node) {
            result.push(rule);
        }
    }
    result
}
