use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

pub fn generate_client_app_id() -> String {
    let mut rng = thread_rng();

    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(7)
        .collect();

    chars
}
