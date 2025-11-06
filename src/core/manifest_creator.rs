use crate::core::deployment::Deployment;
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
}