// Import the damn crate
//

extern crate testaun;
use testaun::testaun_case;

// Those functions are mandatory to be able to use #[testaun]. You'll see an
// error if they're not provided in the current scope.
//

fn testaun_before() {
    println!("[BEFORE]");
}

fn testaun_after() {
    println!("[AFTER]");
}

// Try it out just like below in order to see [BEFORE] and [AFTER] printed out:
// $ cargo test -- --nocapture
//

#[test]
#[testaun_case]
fn should_work_fine() {
    assert!(1 == 1, "oh damn! it should be the other way around");
}

#[test]
fn should_work_as_normal() {
    assert_eq!(1, 1);
}