extern crate uzers;
use uzers::{all_users, User};

extern crate env_logger;

fn main() {
    env_logger::init();

    // We allow unused_unsafe here because the call is unsafe only on some targets.
    #[allow(unused_unsafe)]
    let mut users: Vec<User> = unsafe { all_users() }.collect();
    users.sort_by(|a, b| a.uid().cmp(&b.uid()));

    for user in users {
        println!(
            "User {} has name {}",
            user.uid(),
            user.name().to_string_lossy()
        );
    }
}
