use std::fmt;
use std::fmt::Display;

const NOMINATIVE: &str = "nominative";
const GENITIVE: &str = "genitive";
const DATIVE: &str = "dative";
const ACCUSATIVE: &str = "accusative";
const INSTRUMENTAL: &str = "instrumental";
const PREPOSITIONAL: &str = "prepositional";

//Grammar case constants
#[derive(PartialEq, Debug)]
pub enum Case {
    //именительный
    Nominative,
    //родительный
    Genitive,
    //дательный
    Dative,
    //винительный
    Accusative,
    //творительный
    Instrumental,
    //предложный
    Prepositional,
}

impl Case {
    pub fn value(&self) -> &'static str {
        match self {
            Case::Nominative => NOMINATIVE,
            Case::Genitive => GENITIVE,
            Case::Dative => DATIVE,
            Case::Accusative => ACCUSATIVE,
            Case::Instrumental => INSTRUMENTAL,
            Case::Prepositional => PREPOSITIONAL,
        }
    }
}

impl Display for Case {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.value())
    }
}
