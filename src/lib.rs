pub mod core;
pub mod stringifyer;

#[cfg(test)]
mod tests {
    use crate::core::deployment::Deployment;
    use crate::core::service::Service;
    use crate::stringifyer::build_string;

    #[test]
    fn test_build_string_with_int() {
        let result = build_string(42);
        assert_eq!(result, "42");
    }

    #[test]
    fn test_build_string_with_float() {
        let result = build_string(3.14);
        assert_eq!(result, "3.14");
    }

    #[test]
    fn test_build_string_with_service() {
        let result = build_string(Service::new("test".to_string(), "testdeployment".to_string()));
        println!("{}", result);
    }

    #[test]
    fn test_build_string_with_deployment() {
        let result = build_string(
            Deployment::new(
                "test deployment".to_string(),
                "python:3".to_string()
            )
        );
        println!("{}", result);
    }
}