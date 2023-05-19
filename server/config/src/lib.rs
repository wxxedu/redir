pub trait Config {
    fn get_base_url(&self) -> &str;
    fn get_hash_cost(&self) -> u32;
}
