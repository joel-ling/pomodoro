use chrono::naive::NaiveDate;
use serde::Deserialize;
use serde_yaml::from_str;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type")]
enum Distribution {
    Continuous { alpha: NaiveDate, omega: NaiveDate },
    Discrete { dates: Vec<NaiveDate> },
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
enum Effort {
    Absolute(f64),
    Relative(f64),
}

#[derive(Debug, Deserialize, PartialEq)]
struct Responsibility {
    account: String,
    description: String,
    distribution: Distribution,
    effort: Effort,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialise_responsibility() {
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

        let actual: Responsibility;
        actual = from_str(include_str!("test_responsibility.yml")).unwrap();

        assert_eq!(actual, expected);
    }
}
