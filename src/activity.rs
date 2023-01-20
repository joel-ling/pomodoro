use crate::responsibility::Effort;
use crate::responsibility::Responsibility;

#[derive(Debug, PartialEq)]
pub struct Activity<'a> {
    pub account: &'a String,
    pub description: &'a String,
    pub absolute_effort: f64,
}

impl<'a> Activity<'a> {
    pub fn from(r: &Responsibility, tot_rel_eff: f64, bal_abs_eff: f64, jitter: f64) -> Activity {
        let absolute_effort: f64 = match r.effort {
            Effort::Absolute(effort) => effort,
            Effort::Relative(effort) => (effort + jitter) / tot_rel_eff * bal_abs_eff,
        };

        Activity {
            account: &r.account,
            description: &r.description,
            absolute_effort: absolute_effort,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::from_str;
    use super::*;

    #[test]
    fn from_absolute() {
        let expected = Activity {
            account: &String::from("Team meetings"),
            description: &String::from("Weekly team meeting"),
            absolute_effort: 1.0,
        };

        let r: Responsibility;
        r = from_str(include_str!("test_responsibility_absolute.yml")).unwrap();

        let actual = Activity::from(&r, 0.0, 0.0, 0.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn from_relative() {
        let expected = Activity {
            account: &String::from("Non-billable tasks"),
            description: &String::from("Prepare timesheet"),
            absolute_effort: 3.0,
        };

        let r: Responsibility;
        r = from_str(include_str!("test_responsibility_relative.yml")).unwrap();

        let actual = Activity::from(&r, 2.0, 6.0, 0.0);

        assert_eq!(actual, expected);
    }
}
