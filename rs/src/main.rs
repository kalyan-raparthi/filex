mod kit;
use kit::core;

use std::fmt::Result;

fn main() -> Result {
    core::app_start("localhost", "2020").expect("error");
    Ok(())
}