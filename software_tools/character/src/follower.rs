#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Follower {
    pub profile_name: String,
    pub description: Option<String>,
}

impl std::fmt::Display for Follower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.profile_name)?;
        if let Some(description) = &self.description {
            write!(f, " ({})", description)?;
        }
        Ok(())
    }
}

impl Follower {
    pub fn new(profile_name: String, description: Option<String>) -> Self {
        Self {
            profile_name,
            description,
        }
    }
}
