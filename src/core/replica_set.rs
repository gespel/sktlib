use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::core::models::{Container, Env, Labels, MatchLabels, Selector, Spec, Template, TemplateSpec, Metadata};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ReplicaSet {
    #[serde(rename = "apiVersion")]
    api_version: String,
    kind: String,
    metadata: Metadata,
    spec: Spec
}

impl fmt::Display for ReplicaSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yml::to_string(self).unwrap())
    }
}

impl ReplicaSet {
    pub fn new(name: String, image: String) -> ReplicaSet {
        ReplicaSet {
            api_version: "apps/v1".to_string(),
            kind: "ReplicaSet".to_string(),
            metadata: Metadata {
                name: name.clone(),
                labels: Labels {
                    name: name.clone()
                }
            },
            spec: Spec {
                replicas: 3,
                selector: Selector {
                    match_labels: MatchLabels {
                        name: name.clone()
                    }
                },
                template: Template {
                    metadata: Metadata {
                        labels: Labels {
                            name: name.clone()
                        },
                        name: name.clone(),
                    },
                    spec: TemplateSpec {
                        containers: vec![
                            Container {
                                name: name.clone(),
                                image,
                                env: vec![
                                    Env {
                                        name: "TEST_VAR".to_string(),
                                        value: "asdasd".to_string(),
                                    }
                                ],
                            }
                        ],
                    }
                },
            },
        }
    }
}