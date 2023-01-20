use rand::distributions::uniform::Uniform;
use rand::distributions::Distribution;
use rand::rngs::ThreadRng;

pub fn jitter(original: &Vec<f64>) -> Vec<f64> {
    let mut jittered: Vec<f64> = Vec::new();

    if original.len() == 0 {
        return jittered;
    }

    let compare = |a: &&f64, b: &&f64| a.partial_cmp(b).unwrap();
    // this works as long as neither a nor b is f64::NAN

    let min: f64 = match original.iter().min_by(compare) {
        Some(min) => *min,
        None => 0.0, // should not occur since original.len() > 0 ensured above
    };

    let distribution = Uniform::from(-1.0 * min..min);

    let mut generator = ThreadRng::default();

    let mut jitter: Vec<f64> = Vec::new();

    for _ in 0..original.len() {
        jitter.push(distribution.sample(&mut generator));
    }

    let discrepancy: f64 = jitter.iter().sum();

    let correction: f64 = discrepancy / original.len() as f64;

    for i in 0..original.len() {
        jittered.push(original[i] + jitter[i] - correction);
    }

    jittered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let original: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];

        let jittered: Vec<f64> = jitter(&original);

        assert_eq!(jittered.iter().sum::<f64>(), original.iter().sum());

        for i in &jittered {
            assert!(*i >= 0.0);
        }
    }
}
