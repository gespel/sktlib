use crate::core::deployment::Deployment;
use crate::core::service::Service;
use crate::stringifyer::build_string;

pub struct ManifestCreator {

}

impl ManifestCreator {
    pub fn new() -> ManifestCreator {
        ManifestCreator {}
    }

    pub fn create_deployment(&self, name: &str, image: &str) -> Result<String, String> {
        let deployment = Deployment::new(name.to_string(), image.to_string());
        Ok(build_string(deployment))
    }

    pub fn create_service(&self, name: &str, target: &str, port: i32) -> Result<String, String> {
        let service = Service::new(name.to_string(), target.to_string());
        Ok(build_string(service))
    }
}