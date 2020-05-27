use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Level {
    pub title: String,
    pub branch: String,
    pub solution_checker: String,
    pub flags: Vec<String>,
}

impl fmt::Display for Level {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.title)
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GameConfig {
    pub levels: Vec<Level>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_display() {
        let level = Level {
            title: "tit".to_string(),
            branch: "bra".to_string(),
            solution_checker: "sol".to_string(),
            flags: vec!["fla".to_string()],
        };
        assert_eq!(format!("{}", level), "tit".to_string());
    }
}
