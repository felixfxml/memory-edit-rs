extern crate winapi;

mod process;

fn main() {
    //println!("{}", std::env::consts::OS);

    //println!("{}", &*"1".to_owned() + &*"2" + "3");

    process::run();
}
