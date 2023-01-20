use crate::activity::Activity;
use crate::jitter::jitter;
use crate::responsibility::Distribution;
use crate::responsibility::Effort;
use crate::responsibility::Responsibility;
use chrono::naive::NaiveDate;

#[derive(Debug, PartialEq)]
pub struct DayAtWork<'a> {
    activities: Vec<Activity<'a>>,
    date: NaiveDate,
    total_effort: f64,
}

impl<'a> DayAtWork<'a> {
    pub fn new(
        date: NaiveDate,
        tot_abs_eff: f64,
        resolution: f64,
        responsibilities: &Vec<Responsibility>,
    ) -> DayAtWork {
        let mut day = DayAtWork {
            activities: Vec::new(),
            date: date,
            total_effort: 0.0,
        };

        let mut relevant: Vec<&Responsibility> = Vec::new();

        for r in responsibilities {
            match &r.distribution {
                Distribution::Continuous { alpha, omega } => {
                    if date < *alpha || date > *omega {
                        continue;
                    }
                }
                Distribution::Discrete { dates } => {
                    if !dates.contains(&date) {
                        continue;
                    }
                }
            }

            relevant.push(&r)
        }

        let mut tot_rel_eff: f64 = 0.0;
        let mut bal_abs_eff: f64 = tot_abs_eff;

        let mut jitter_source: Vec<f64> = Vec::new();

        for r in &relevant {
            match r.effort {
                Effort::Absolute(effort) => bal_abs_eff -= effort,
                Effort::Relative(effort) => {
                    tot_rel_eff += effort;

                    jitter_source.push(effort);
                }
            }
        }

        let mut effort_jitter: Vec<f64> = jitter(&jitter_source);

        for r in relevant {
            let activity = Activity::from(
                r,
                tot_rel_eff,
                bal_abs_eff,
                match r.effort {
                    Effort::Absolute(_) => 0.0,
                    Effort::Relative(_) => effort_jitter.pop().unwrap(),
                },
                resolution,
            );

            day.total_effort += activity.absolute_effort;

            day.activities.push(activity);
        }

        day
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml::from_str;

    #[test]
    fn new() {
        let account0 = String::from("Team meetings");
        let account1 = String::from("Non-billable tasks");

        let description0 = String::from("Weekly team meeting");
        let description1 = String::from("Prepare timesheet");

        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();

        let expected = DayAtWork {
            activities: vec![
                Activity {
                    account: &account0,
                    description: &description0,
                    absolute_effort: 1.0,
                },
                Activity {
                    account: &account1,
                    description: &description1,
                    absolute_effort: 7.0,
                },
            ],
            date: date.clone(),
            total_effort: 8.0,
        };

        let yaml_abs: &str = include_str!("test_responsibility_absolute.yml");
        let yaml_rel: &str = include_str!("test_responsibility_relative.yml");

        let responsibilities: Vec<Responsibility> =
            vec![from_str(yaml_abs).unwrap(), from_str(yaml_rel).unwrap()];

        let actual = DayAtWork::new(date, 8.0, 0.25, &responsibilities);

        assert_eq!(actual, expected);
    }
}
