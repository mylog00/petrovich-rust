use crate::case::Case;
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
    fn of(rules: &Yaml) -> RuleGroup {
        let exceptions = to_rule_vec(&rules["exceptions"]);
        let suffixes = to_rule_vec(&rules["suffixes"]);
        RuleGroup {
            exceptions,
            suffixes,
        }
    }

    pub fn get_rule(&self, word: &str, gender: &Gender, first_word: bool) -> Option<&Rule> {
        let mut rule = RuleGroup::find_rule(&self.exceptions, word, gender, first_word);
        if rule.is_none() {
            rule = RuleGroup::find_rule(&self.suffixes, word, gender, first_word);
        }
        rule
    }

    fn find_rule<'a>(
        rules: &'a [Rule],
        word: &str,
        gender: &Gender,
        first_word: bool,
    ) -> Option<&'a Rule> {
        for rule in rules {
            if rule.match_rule(word, gender, first_word) {
                return Some(&rule);
            }
        }
        None
    }
}

#[derive(PartialEq, Debug)]
pub struct Rule {
    gender: Gender,
    test: Vec<String>,
    mods: Vec<String>,
    first_word: bool,
}

impl Rule {
    fn of(rules: &Yaml) -> Option<Rule> {
        let gender = Gender::of(&rules["gender"].as_str()?)?;
        let test = to_str_vec(&rules["test"]);
        let mods = to_str_vec(&rules["mods"]);
        let first_word = !rules["tags"].is_badvalue();

        let rule = Rule {
            gender,
            test,
            mods,
            first_word,
        };
        Some(rule)
    }

    fn match_rule(&self, word: &str, gender: &Gender, is_first_word: bool) -> bool {
        if !gender.equal(&self.gender) {
            return false;
        };
        // If property `first_word = true`
        // that means we can apply that rule for first word only.
        // If property `first_word = false`
        // that means we can apply that rule for any word.
        if self.first_word && !is_first_word  {
            return false;
        }
        for val in &self.test {
            if word.ends_with(val) {
                return true;
            }
        }
        false
    }

    pub fn apply(&self, word: &str, case: &Case) -> String {
        let modifier = self.get_modifier(case);
        match modifier {
            None => word.to_string(),
            Some(modifier) => apply_modifier(word, modifier),
        }
    }

    fn get_modifier(&self, case: &Case) -> Option<&String> {
        match case {
            Case::Nominative => Option::None,
            Case::Genitive => self.mods.get(0),
            Case::Dative => self.mods.get(1),
            Case::Accusative => self.mods.get(2),
            Case::Instrumental => self.mods.get(3),
            Case::Prepositional => self.mods.get(4),
        }
    }
}

fn apply_modifier(name: &str, modifier: &str) -> String {
    let mut name = String::from(name);
    let mut postfix = String::new();
    modifier.chars().for_each(|c| match c {
        '-' => {
            name.pop();
        }
        '.' => {}
        _ => postfix.push(c),
    });
    name.push_str(&postfix);
    name
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_rule_test() {
        let rules = &test_rules();

        let rule_group = rules.first_name();
        let expected = rule_group.exceptions.get(0).unwrap();
        let actual = rule_group.get_rule("лев", &Gender::Male, false);
        assert_eq!(Some(expected), actual);

        let rule_group = rules.middle_name();
        let expected = rule_group.exceptions.get(0).unwrap();
        let actual = rule_group.get_rule("борух", &Gender::Male, true);
        assert_eq!(Some(expected), actual);

        let rule_group = rules.last_name();
        let actual = rule_group.get_rule("абдул", &Gender::Male, false);
        assert_eq!(None, actual);
    }

    #[test]
    fn match_rule_test() {
        let rule = Rule {
            gender: Gender::Female,
            test: vec!["кс".to_string(), "пс".to_string(), "мс".to_string()],
            mods: vec![],
            first_word: false,
        };
        let case1 = "прокс";
        let case2 = "пс";
        let case3 = "бромс";
        //positive
        assert!(rule.match_rule(case1, &Gender::Androgynous, false));
        assert!(rule.match_rule(case1, &Gender::Androgynous, true));
        assert!(rule.match_rule(case2, &Gender::Female, false));
        assert!(rule.match_rule(case3, &Gender::Female, true));
        //negative
        assert!(!rule.match_rule(case1, &Gender::Male, false));
        assert!(!rule.match_rule(case2, &Gender::Male, false));
        assert!(!rule.match_rule(case3, &Gender::Male, false));
        assert!(!rule.match_rule(case1, &Gender::Male, true));
        assert!(!rule.match_rule(case2, &Gender::Male, true));
        assert!(!rule.match_rule(case3, &Gender::Male, true));
        assert!(!rule.match_rule("с", &Gender::Female, false));
        assert!(!rule.match_rule("с", &Gender::Female, true));
        assert!(!rule.match_rule("шпротс", &Gender::Androgynous, false));
        assert!(!rule.match_rule("шпротс", &Gender::Androgynous, true));
        assert!(!rule.match_rule("петрович", &Gender::Male, false));
        assert!(!rule.match_rule("петрович", &Gender::Male, true));
    }
    #[test]
    fn match_rule_first_word_test() {
        let rule = Rule {
            gender: Gender::Male,
            test: vec!["а".to_string(), "е".to_string(), "и".to_string()],
            mods: vec![],
            first_word: true,
        };
        let case1 = "сова";
        let case2 = "е";
        let case3 = "мыши";
        //positive
        assert!(rule.match_rule(case1, &Gender::Male, true));
        assert!(rule.match_rule(case2, &Gender::Androgynous, true));
        assert!(rule.match_rule(case3, &Gender::Male, true));

        //negative
        assert!(!rule.match_rule(case1, &Gender::Male, false));
        assert!(!rule.match_rule(case2, &Gender::Male, false));
        assert!(!rule.match_rule(case3, &Gender::Androgynous, false));

        assert!(!rule.match_rule(case1, &Gender::Female, true));
        assert!(!rule.match_rule(case2, &Gender::Female, true));
        assert!(!rule.match_rule(case3, &Gender::Female, true));

        assert!(!rule.match_rule("ис", &Gender::Female, false));
        assert!(!rule.match_rule("ис", &Gender::Female, true));
        assert!(!rule.match_rule("дрозд", &Gender::Androgynous, false));
        assert!(!rule.match_rule("дрозд", &Gender::Androgynous, true));
        assert!(!rule.match_rule("мидвед", &Gender::Male, false));
        assert!(!rule.match_rule("мидвед", &Gender::Male, true));
    }
    #[test]
    fn get_modifier_test() {
        let rules = test_rules();
        let rule = rules.first_name().exceptions.get(0).unwrap();

        let actual = rule.get_modifier(&Case::Nominative);
        assert_eq!(None, actual);
        let actual = rule.get_modifier(&Case::Genitive);
        assert_eq!(Some(&"--ьва".to_string()), actual);
        let actual = rule.get_modifier(&Case::Dative);
        assert_eq!(Some(&"--ьву".to_string()), actual);
        let actual = rule.get_modifier(&Case::Accusative);
        assert_eq!(Some(&"--ьва".to_string()), actual);
        let actual = rule.get_modifier(&Case::Instrumental);
        assert_eq!(Some(&"--ьвом".to_string()), actual);
        let actual = rule.get_modifier(&Case::Prepositional);
        assert_eq!(Some(&"--ьве".to_string()), actual);
    }
    #[test]
    fn apply_modifier_test() {
        let actual = apply_modifier("Маша", "-и");
        assert_eq!("Маши", actual);

        let actual = apply_modifier("пётр", "---етру");
        assert_eq!("петру", actual);

        let actual = apply_modifier("Маша", ".");
        assert_eq!("Маша", actual);

        let actual = apply_modifier("", "-и");
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
