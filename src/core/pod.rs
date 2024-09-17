use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::core::models::{Container, Env, Labels, Metadata, TemplateSpec};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Pod {
    #[serde(rename = "apiVersion")]
    api_version: String,
    kind: String,
    metadata: Metadata,
    spec: TemplateSpec
}

impl fmt::Display for Pod {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yml::to_string(self).unwrap())
    }
}

impl Pod {
    pub fn new(name: String, image_name: String) -> Pod {
        Pod {
            api_version: "v1".to_string(),
            kind: "Pod".to_string(),
            metadata: Metadata {
                labels: Labels {
                    name: name.clone()
                },
                name: name.clone()
            },
            spec: TemplateSpec {
                containers: vec![
                    Container {
                        name: name.clone(),
                        image: image_name.clone(),
                        env: vec![
                            Env {
                                name: "TESTVAR".to_string(),
                                value: "asd".to_string(),
                            }
                        ],
                    },
                ],
            },
        }
    }
}