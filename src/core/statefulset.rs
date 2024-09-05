use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::core::models::{Container, Env, Labels, MatchLabels, Selector, Spec, Template, TemplateSpec, Metadata};

/*apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: web
spec:
  selector:
    matchLabels:
      app: nginx # has to match .spec.template.metadata.labels
  serviceName: "nginx"
  replicas: 3 # by default is 1
  minReadySeconds: 10 # by default is 0
  template:
    metadata:
      labels:
        app: nginx # has to match .spec.selector.matchLabels
    spec:
      terminationGracePeriodSeconds: 10
      containers:
      - name: nginx
        image: registry.k8s.io/nginx-slim:0.24
        ports:
        - containerPort: 80
          name: web
        volumeMounts:
        - name: www
          mountPath: /usr/share/nginx/html
  volumeClaimTemplates:
  - metadata:
      name: www
    spec:
      accessModes: [ "ReadWriteOnce" ]
      storageClassName: "my-storage-class"
      resources:
        requests:
          storage: 1Gi*/
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct StatefulSet {
    #[serde(rename = "apiVersion")]
    api_version: String,
    kind: String,
    metadata: Metadata,
    spec: Spec
}

impl fmt::Display for StatefulSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_yml::to_string(self).unwrap())
    }
}

impl StatefulSet {
    pub fn new(name: String, image: String) -> StatefulSet {
        StatefulSet {
            api_version: "apps/v1".to_string(),
            kind: "StatefulSet".to_string(),
            metadata: Metadata {
                name: name.clone(),
                labels: Labels {
                    name: name.clone()
                }
            },
            spec: Spec {
                replicas: 1,
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