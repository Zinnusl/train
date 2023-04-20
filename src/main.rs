#![feature(is_some_and)]
#[allow(unused_imports)]

use async_std::task::block_on;
use std::sync::RwLock;

use std::cell::RefCell;

use di::*;
use std::rc::Rc;

mod abst;
mod imp;
mod util;

use abst::*;
use imp::*;

fn main() -> Result<(), ()> {
    let mut services = ServiceCollection::new();

    let provider = services.build_provider().unwrap();


    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use pretty_assertions::{assert_eq, assert_ne};
// }
