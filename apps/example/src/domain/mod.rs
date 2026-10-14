use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Greeting {
    pub message: String,
}

impl Greeting {
    pub fn for_name(name: &str) -> Self {
        let name = if name.trim().is_empty() {
            "world"
        } else {
            name.trim()
        };
        Self {
            message: format!("Hello, {name}!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Greeting;

    #[test]
    fn uses_world_when_name_is_empty() {
        assert_eq!(Greeting::for_name(" ").message, "Hello, world!");
    }

    #[test]
    fn trims_name() {
        assert_eq!(Greeting::for_name(" developer ").message, "Hello, developer!");
    }
}
