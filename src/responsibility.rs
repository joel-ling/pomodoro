use chrono::naive::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Distribution {
    Continuous { alpha: NaiveDate, omega: NaiveDate },
    Discrete { dates: Vec<NaiveDate> },
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Effort {
    Absolute(f64),
    Relative(f64),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Responsibility {
    pub account: String,
    pub description: String,
    pub distribution: Distribution,
    pub effort: Effort,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml::from_str;

    #[test]
    fn deserialise() {
        let expected = Responsibility {
            account: String::from("Team meetings"),
            description: String::from("Weekly team meeting"),
            distribution: Distribution::Discrete {
                dates: vec![
                    NaiveDate::from_ymd_opt(2022, 12, 25).unwrap(),
                    NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                ],
            },
            effort: Effort::Absolute(1.0),
        };

        let yaml: &str = include_str!("test_responsibility_absolute.yml");

        let actual: Responsibility = from_str(yaml).unwrap();

        assert_eq!(actual, expected);
    }
}
