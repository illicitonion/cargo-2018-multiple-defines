use ::ignore;

pub fn f() {
    for result in ignore::Walk::new(".") {
        println!("{:?}", result);
    }
}
