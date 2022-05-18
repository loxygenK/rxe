use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

pub fn get_random_string() -> String {
    let mut rng = thread_rng();

    (0..15).map(|_| rng.sample(Alphanumeric) as char).collect()
}
