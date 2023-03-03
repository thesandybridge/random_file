use rand::{distributions::Alphanumeric, Rng};
/// Generate a random string of Alphanumeric characters.
///
/// # Arguments
///
/// * `length` - The length of the string. 
///
/// # Examples
///
/// ```
/// let first = gf::generate_string(8);
/// let second = gf::generate_string(8);
/// assert_ne!(first, second);
/// ```
pub fn generate_string(length: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    return s;
}
