
use lib::example;
mod lib;

#[bench]
pub fn are_we_fast_yet() {
    example();
}
