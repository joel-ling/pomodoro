use crate::responsibility::Effort;
use crate::responsibility::Responsibility;
use serde_yaml::from_str;

#[derive(Debug, PartialEq)]
struct Activity<'a> {
    account: &'a String,
    description: &'a String,
    absolute_effort: f64,
}

impl<'a> Activity<'a> {
    fn from(r: &Responsibility, tot_rel_eff: f64, bal_abs_eff: f64) -> Activity {
        Activity {
            account: &r.account,
            description: &r.description,
            absolute_effort: match r.effort {
                Effort::Absolute(effort) => effort,
                Effort::Relative(effort) => effort / tot_rel_eff * bal_abs_eff,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_absolute() {
        let expected = Activity {
            account: &String::from("Team meetings"),
            description: &String::from("Weekly team meeting"),
            absolute_effort: 1.0,
        };

        let r: Responsibility;
        r = from_str(include_str!("test_responsibility.yml")).unwrap();

        let actual = Activity::from(&r, 0.0, 0.0);

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

        let actual = Activity::from(&r, 2.0, 6.0);

        assert_eq!(actual, expected);
    }
}
