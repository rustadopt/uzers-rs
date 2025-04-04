extern crate uzers;

#[cfg(feature = "test-integration")]
mod integration {
    #[test]
    #[serial_test::serial]
    fn test_group_by_name() {
        let group = uzers::get_group_by_name("bosses");

        assert_eq!(group.is_some(), true);

        let group = group.unwrap();

        assert_eq!(group.gid(), 42);
        assert_eq!(group.name(), "bosses");
    }

    #[test]
    #[serial_test::serial]
    fn test_all_groups() {
        let groups: Vec<_> = unsafe { uzers::all_groups() }.collect();
        assert_eq!(groups.len(), 2);

        let group = &groups[0];
        assert_eq!(group.gid(), 42);
        assert_eq!(group.name(), "bosses");

        let group = &groups[1];
        assert_eq!(group.gid(), 43);
        assert_eq!(group.name(), "contributors");
    }
}
