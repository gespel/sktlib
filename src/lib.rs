pub mod core;
pub mod stringifyer;

#[cfg(test)]
mod tests {
    use crate::core::daemon_set::DaemonSet;
    use crate::core::deployment::Deployment;
    use crate::core::pod::Pod;
    use crate::core::replica_set::ReplicaSet;
    use crate::core::service::Service;
    use crate::core::stateful_set::StatefulSet;
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
        let result = build_string(
            Service::new(
                "test".to_string(),
                "testdeployment".to_string()
            )
        );
        println!("{}", result);
    }

    #[test]
    fn test_build_string_with_deployment() {
        let result = build_string(
            Deployment::new(
                "testdeployment".to_string(),
                "python:3".to_string()
            )
        );
        println!("{}", result);
    }

    #[test]
    fn test_build_string_with_stateful_set() {
        let result = build_string(
            StatefulSet::new(
                "testdeployment".to_string(),
                "python3".to_string()
            )
        );
        println!("{}", result);
    }

    #[test]
    fn test_build_string_with_pod() {
        let result = build_string(
            Pod::new(
                "test".to_string(),
                "testdeployment".to_string()
            )
        );
        println!("{}", result);
    }

    #[test]
    fn test_build_string_with_daemon_set() {
        let result = build_string(
            DaemonSet::new(
                "test".to_string(),
                "testdeployment".to_string()
            )
        );
        println!("{}", result);
    }

    #[test]
    fn test_build_string_with_replica_set() {
        let result = build_string(
            ReplicaSet::new(
                "test".to_string(),
                "testdeployment".to_string()
            )
        );
        println!("{}", result);
    }
}