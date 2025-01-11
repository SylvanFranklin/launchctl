use crate::Service;

#[cfg(test)]
mod tests {
    use super::*;

    // make these tests more comprehensive
    #[test]
    fn creation_basic() {
        let service = Service::builder()
            .name("com.<owner name>.<binary name>")
            .build();

        assert_eq!(service.name, "com.<owner name>.<binary name>");
        assert_eq!(service.uid, "501");
        assert_eq!(service.domain_target, "gui/501");
        assert_eq!(
            service.service_target,
            "gui/501/com.<owner name>.<binary name>"
        );
        assert_eq!(
            service.plist_path,
            "~/Library/LaunchAgents/com.<owner name>.<binary name>.plist"
        );
        assert_eq!(
            service.error_log_path,
            "/tmp/com.<owner name>.<binary name>_501.err.log"
        );
        assert_eq!(
            service.out_log_path,
            "/tmp/com.<owner name>.<binary name>_501.out.log"
        );
    }
    #[test]
    fn creation_advanced() {
        let service = Service::builder()
            .name("some weird unconventional name")
            .uid("401")
            .build();

        assert_eq!(service.name, "some weird unconventional name");
        assert_eq!(service.uid, "401");
        assert_eq!(service.domain_target, "gui/401");
        assert_eq!(
            service.service_target,
            "gui/401/some weird unconventional name"
        );
        assert_eq!(
            service.plist_path,
            "~/Library/LaunchAgents/some weird unconventional name.plist"
        );
        assert_eq!(
            service.error_log_path,
            "/tmp/some weird unconventional name_401.err.log"
        );
        assert_eq!(
            service.out_log_path,
            "/tmp/some weird unconventional name_401.out.log"
        );
    }
}
