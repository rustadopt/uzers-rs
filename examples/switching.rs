extern crate uzers;
use std::mem::drop;
use uzers::switch::switch_user_group;
use uzers::{get_current_gid, get_current_uid, get_effective_gid, get_effective_uid, uid_t};

extern crate env_logger;

const SAMPLE_ID: uid_t = 502;

fn main() {
    env_logger::init();

    println!("\nInitial values:");
    print_state();

    println!("\nValues after switching:");
    let guard = switch_user_group(SAMPLE_ID, SAMPLE_ID);
    print_state();

    println!("\nValues after switching back:");
    drop(guard);
    print_state();
}

fn print_state() {
    println!(
        "Current UID/GID: {}/{}",
        get_current_uid(),
        get_current_gid()
    );
    println!(
        "Effective UID/GID: {}/{}",
        get_effective_uid(),
        get_effective_gid()
    );
}
