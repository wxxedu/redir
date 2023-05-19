pub trait Config {
    /// Returns the base URL.
    fn get_base_url(&self) -> &str;

    /// Returns the hash cost.
    fn get_hash_cost(&self) -> u32;
}
