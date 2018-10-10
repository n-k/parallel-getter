extern crate parallel_getter;

use parallel_getter::ParallelGetter;
use std::fs::File;
use std::sync::Arc;

fn main() {
    let url = "http://apt.pop-os.org/proprietary/pool/bionic/main/\
        binary-amd64/a/atom/atom_1.31.1_amd64.deb";
    let mut file = File::create("atom_1.31.1_amd64.deb").unwrap();
    let result = ParallelGetter::new(url, &mut file)
        .threads(4)
        .callback(1000, Arc::new(|p, t| {
            println!("{} KiB out of {} KiB", p / 1024, t / 1024);
        }))
        .get();

    if let Err(why) = result {
        eprintln!("errored: {}", why);
    }
}