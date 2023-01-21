use rand::distributions::uniform::Uniform;
use rand::distributions::Distribution;
use rand::rngs::ThreadRng;

pub fn jitter(source: &Vec<f64>) -> Vec<f64> {
    let mut output: Vec<f64> = Vec::new();

    if source.len() == 0 {
        return output;
    }

    let compare = |a: &&f64, b: &&f64| a.partial_cmp(b).unwrap();
    // this works as long as neither a nor b is f64::NAN

    let min: f64 = match source.iter().min_by(compare) {
        Some(min) => *min,
        None => 0.0, // should not occur since source.len() > 0 ensured above
    };

    let distribution = Uniform::from(-1.0 * min..min);

    let mut generator = ThreadRng::default();

    for _ in 0..source.len() {
        output.push(distribution.sample(&mut generator));
    }

    let discrepancy: f64 = output.iter().sum();

    let correction: f64 = discrepancy / source.len() as f64;

    for i in 0..source.len() {
        output[i] -= correction;
        // XXX: probability of correction resulting in output exceeding range
        // is non-zero, necessitating fix below

        if output[i] < -1.0 * min || output[i] > min {
            return jitter(source);
            // XXX: discarding of undesirable output probably distorts
            // distribution of random numbers
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let source: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];

        let jitter: Vec<f64> = jitter(&source);

        assert!(jitter.iter().sum::<f64>() < 0.001);

        for i in &jitter {
            assert!(*i >= -1.0 && *i <= 1.0);
        }
    }
}
