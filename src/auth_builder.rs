pub trait IntoAuthBuilder {
    fn get_host(&self) -> Result<String, anyhow::Error>;
    fn get_token(&self) -> Result<String, anyhow::Error>;
}
