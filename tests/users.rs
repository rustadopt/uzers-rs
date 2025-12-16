extern crate uzers;

#[cfg(feature = "test-integration")]
mod integration {
    use std::path::PathBuf;

    use uzers::os::unix::UserExt;

    #[test]
    #[serial_test::serial]
    fn test_user_by_name() {
        let user = uzers::get_user_by_name("fred");

        assert_eq!(user.is_some(), true);

        let user = user.unwrap();

        assert_eq!(user.uid(), 1337);
        assert_eq!(user.name(), "fred");
        assert_eq!(user.primary_group_id(), 42);
        assert_eq!(user.home_dir(), PathBuf::from("/home/fred"));
    }

    #[cfg(target_os = "linux")]
    #[test]
    #[serial_test::serial]
    fn test_all_users_from_file() {
        let test_passwd_file_path = std::env::var("NSS_WRAPPER_PASSWD").unwrap();

        let users: Vec<_> = uzers::all_users_from_file(test_passwd_file_path).collect();
        assert_eq!(users.len(), 1);

        let user = users.first().unwrap();
        assert_eq!(user.uid(), 1337);
        assert_eq!(user.name(), "fred");
        assert_eq!(user.primary_group_id(), 42);
        assert_eq!(user.home_dir(), PathBuf::from("/home/fred"));
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    #[serial_test::serial]
    fn test_all_users() {
        let users: Vec<_> = unsafe { uzers::all_users() }.collect();
        assert_eq!(users.len(), 1);

        let user = users.first().unwrap();
        assert_eq!(user.uid(), 1337);
        assert_eq!(user.name(), "fred");
        assert_eq!(user.primary_group_id(), 42);
        assert_eq!(user.home_dir(), PathBuf::from("/home/fred"));
    }
}
