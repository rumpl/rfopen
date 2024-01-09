use std::fs::File;

#[allow(unused_must_use)]
fn main() {
    for _ in 0..10000 {
        File::open("foo.txt");
    }
}
