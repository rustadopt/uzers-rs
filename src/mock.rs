//! Mockable users and groups.
//!
//! When you’re testing your code, you don’t want to actually rely on the
//! system actually having various users and groups present - it’s much better
//! to have a custom set of users that are *guaranteed* to be there, so you can
//! test against them.
//!
//! This module allows you to create these custom users and groups
//! definitions, then access them using the same `Users` trait as in the main
//! library, with few changes to your code.
//!
//!
//! ## Creating Mock Users
//!
//! The only thing a mock users table needs to know in advance is the UID of
//! the current user. Aside from that, you can add users and groups with
//! `add_user` and `add_group` to the table:
//!
//! ```
//! use std::sync::Arc;
//! use uzers::mock::{MockUsers, User, Group};
//! use uzers::os::unix::{UserExt, GroupExt};
//!
//! let mut users = MockUsers::with_current_uid(1000);
//! let bobbins = User::new(1000, "Bobbins", 1000).with_home_dir("/home/bobbins");
//! users.add_user(bobbins);
//! users.add_group(Group::new(100, "funkyppl"));
//! ```
//!
//! The exports get re-exported into the mock module, for simpler `use` lines.
//!
//!
//! ## Using Mock Users
//!
//! To set your program up to use either type of `Users` table, make your
//! functions and structs accept a generic parameter that implements the `Users`
//! trait. Then, you can pass in a value of either Cache, Snapshot or Mock type.
//!
//! Here’s a complete example:
//!
//! ```
//! use std::sync::Arc;
//! use uzers::{Users, UsersCache, User};
//! use uzers::os::unix::UserExt;
//! use uzers::mock::MockUsers;
//!
//! fn print_current_username<U: Users>(users: &mut U) {
//!     println!("Current user: {:?}", users.get_current_username());
//! }
//!
//! let mut users = MockUsers::with_current_uid(1001);
//! users.add_user(User::new(1001, "fred", 101));
//! print_current_username(&mut users);
//!
//! let mut actual_users = UsersCache::new();
//! print_current_username(&mut actual_users);
//! ```
//!
//! Include `AllUsers` in generic parameter bounds if iteration is required:
//!
//! ```
//! use uzers::{AllUsers, User, Users, UsersSnapshot};
//! use uzers::mock::MockUsers;
//!
//! fn print_all_users<U: Users + AllUsers>(users: &U) {
//!     println!("All users:");
//!     for user in users.get_all_users() {
//!         println!("- {:?}", user.name());
//!     }
//! }
//!
//! let mut users = MockUsers::with_current_uid(1001);
//! users.add_user(User::new(1001, "fred", 101));
//! print_all_users(&users);
//!
//! let actual_users = unsafe { UsersSnapshot::new() };
//! print_all_users(&users);

use std::collections::HashMap;
use std::ffi::OsStr;
use std::ops::Deref;
use std::sync::Arc;

pub use base::{Group, User};
pub use libc::{gid_t, uid_t};
pub use traits::{AllGroups, AllUsers, Groups, Users};

/// A mocking users table that you can add your own users and groups to.
pub struct MockUsers {
    users: HashMap<uid_t, Arc<User>>,
    groups: HashMap<gid_t, Arc<Group>>,
    uid: uid_t,
}

impl MockUsers {
    /// Create a new, empty mock users table.
    pub fn with_current_uid(current_uid: uid_t) -> Self {
        Self {
            users: HashMap::new(),
            groups: HashMap::new(),
            uid: current_uid,
        }
    }

    /// Add a user to the users table.
    pub fn add_user(&mut self, user: User) -> Option<Arc<User>> {
        self.users.insert(user.uid(), Arc::new(user))
    }

    /// Add a group to the groups table.
    pub fn add_group(&mut self, group: Group) -> Option<Arc<Group>> {
        self.groups.insert(group.gid(), Arc::new(group))
    }
}

impl Users for MockUsers {
    fn get_user_by_uid(&self, uid: uid_t) -> Option<Arc<User>> {
        self.users.get(&uid).cloned()
    }

    fn get_user_by_name<S: AsRef<OsStr> + ?Sized>(&self, username: &S) -> Option<Arc<User>> {
        self.users
            .values()
            .find(|u| u.name() == username.as_ref())
            .cloned()
    }

    fn get_current_uid(&self) -> uid_t {
        self.uid
    }

    fn get_current_username(&self) -> Option<Arc<OsStr>> {
        self.users
            .get(&self.uid)
            .cloned()
            .map(|u| Arc::clone(&u.name_arc))
    }

    fn get_effective_uid(&self) -> uid_t {
        self.uid
    }

    fn get_effective_username(&self) -> Option<Arc<OsStr>> {
        self.users
            .get(&self.uid)
            .cloned()
            .map(|u| Arc::clone(&u.name_arc))
    }
}

impl Groups for MockUsers {
    fn get_group_by_gid(&self, gid: gid_t) -> Option<Arc<Group>> {
        self.groups.get(&gid).cloned()
    }

    fn get_group_by_name<S: AsRef<OsStr> + ?Sized>(&self, group_name: &S) -> Option<Arc<Group>> {
        self.groups
            .values()
            .find(|g| g.name() == group_name.as_ref())
            .cloned()
    }

    fn get_current_gid(&self) -> uid_t {
        self.uid
    }

    fn get_current_groupname(&self) -> Option<Arc<OsStr>> {
        self.groups
            .get(&self.uid)
            .cloned()
            .map(|u| Arc::clone(&u.name_arc))
    }

    fn get_effective_gid(&self) -> uid_t {
        self.uid
    }

    fn get_effective_groupname(&self) -> Option<Arc<OsStr>> {
        self.groups
            .get(&self.uid)
            .cloned()
            .map(|u| Arc::clone(&u.name_arc))
    }
}

impl AllUsers for MockUsers {
    type UserIter<'a> = std::iter::Map<
        std::collections::hash_map::Values<'a, uid_t, Arc<User>>,
        for<'b> fn(&'b Arc<User>) -> &'b User,
    >;

    fn get_all_users(&self) -> Self::UserIter<'_> {
        self.users.values().map(Arc::deref)
    }
}

impl AllGroups for MockUsers {
    type GroupIter<'a> = std::iter::Map<
        std::collections::hash_map::Values<'a, gid_t, Arc<Group>>,
        for<'b> fn(&'b Arc<Group>) -> &'b Group,
    >;

    fn get_all_groups(&self) -> Self::GroupIter<'_> {
        self.groups.values().map(Arc::deref)
    }
}

#[cfg(test)]
mod test {
    use super::MockUsers;
    use base::{Group, User};
    use traits::{AllGroups, AllUsers, Groups, Users};

    use std::ffi::OsStr;
    use std::sync::Arc;

    #[test]
    fn current_username() {
        let mut users = MockUsers::with_current_uid(1337);
        users.add_user(User::new(1337, "fred", 101));
        assert_eq!(
            Some(Arc::from(OsStr::new("fred"))),
            users.get_current_username()
        )
    }

    #[test]
    fn no_current_username() {
        let users = MockUsers::with_current_uid(1337);
        assert_eq!(None, users.get_current_username())
    }

    #[test]
    fn uid() {
        let mut users = MockUsers::with_current_uid(0);
        users.add_user(User::new(1337, "fred", 101));
        assert_eq!(
            Some(Arc::from(OsStr::new("fred"))),
            users.get_user_by_uid(1337).map(|u| Arc::clone(&u.name_arc))
        )
    }

    #[test]
    fn username() {
        let mut users = MockUsers::with_current_uid(1337);
        users.add_user(User::new(1440, "fred", 101));
        assert_eq!(Some(1440), users.get_user_by_name("fred").map(|u| u.uid()))
    }

    #[test]
    fn no_username() {
        let mut users = MockUsers::with_current_uid(1337);
        users.add_user(User::new(1337, "fred", 101));
        assert_eq!(None, users.get_user_by_name("criminy").map(|u| u.uid()))
    }

    #[test]
    fn no_uid() {
        let users = MockUsers::with_current_uid(0);
        assert_eq!(
            None,
            users.get_user_by_uid(1337).map(|u| Arc::clone(&u.name_arc))
        )
    }

    #[test]
    fn all_users() {
        let mut users = MockUsers::with_current_uid(1337);
        users.add_user(User::new(1337, "fred", 101));
        assert_eq!(
            vec![1337],
            users.get_all_users().map(|u| u.uid()).collect::<Vec<_>>()
        )
    }

    #[test]
    fn gid() {
        let mut users = MockUsers::with_current_uid(0);
        users.add_group(Group::new(1337, "fred"));
        assert_eq!(
            Some(Arc::from(OsStr::new("fred"))),
            users
                .get_group_by_gid(1337)
                .map(|g| Arc::clone(&g.name_arc))
        )
    }

    #[test]
    fn group_name() {
        let mut users = MockUsers::with_current_uid(0);
        users.add_group(Group::new(1337, "fred"));
        assert_eq!(Some(1337), users.get_group_by_name("fred").map(|g| g.gid()))
    }

    #[test]
    fn no_group_name() {
        let mut users = MockUsers::with_current_uid(0);
        users.add_group(Group::new(1337, "fred"));
        assert_eq!(None, users.get_group_by_name("santa").map(|g| g.gid()))
    }

    #[test]
    fn no_gid() {
        let users = MockUsers::with_current_uid(0);
        assert_eq!(
            None,
            users
                .get_group_by_gid(1337)
                .map(|g| Arc::clone(&g.name_arc))
        )
    }

    #[test]
    fn all_groups() {
        let mut users = MockUsers::with_current_uid(1337);
        users.add_group(Group::new(1337, "fred"));
        assert_eq!(
            vec![1337],
            users.get_all_groups().map(|g| g.gid()).collect::<Vec<_>>()
        )
    }

    #[test]
    fn gecos() {
        use crate::os::unix::UserExt;

        let mut users = MockUsers::with_current_uid(1337);
        users.add_user(User::new(1337, "fred", 101).with_gecos("Fred Santa"));

        assert_eq!(
            Some("Fred Santa".to_string()),
            users
                .get_user_by_uid(1337)
                .map(|u| u.gecos().to_string_lossy().to_string())
        );
    }

    #[test]
    fn no_gecos() {
        use crate::os::unix::UserExt;

        let mut users = MockUsers::with_current_uid(1337);
        users.add_user(User::new(1337, "fred", 101));

        assert_eq!(
            Some("".to_string()),
            users
                .get_user_by_uid(1337)
                .map(|u| u.gecos().to_string_lossy().to_string())
        );
    }
}
