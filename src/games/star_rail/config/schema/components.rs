use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use crate::star_rail::consts::launcher_dir;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Components {
    pub path: PathBuf,
    pub servers: Vec<String>
}

impl Default for Components {
    #[inline]
    fn default() -> Self {
        let launcher_dir = launcher_dir().expect("Failed to get launcher dir");

        Self {
            path: launcher_dir.join("components"),
            servers: vec![
                String::from("https://github.com/aaaadev/components")
            ]
        }
    }
}

impl From<&JsonValue> for Components {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            path: match value.get("path") {
                Some(value) => match value.as_str() {
                    Some(value) => PathBuf::from(value),
                    None => default.path
                },
                None => default.path
            },

            servers: match value.get("servers") {
                Some(value) => match value.as_array() {
                    Some(values) => {
                        let mut servers = Vec::new();

                        for value in values {
                            if let Some(server) = value.as_str() {
                                servers.push(server.to_string());
                            }
                        }

                        servers
                    },
                    None => default.servers
                },
                None => default.servers
            }
        }
    }
}
