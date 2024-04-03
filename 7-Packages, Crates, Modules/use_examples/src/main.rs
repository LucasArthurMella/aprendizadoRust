use std::collections::HashMap;
fn main(){
    let mut map = HashMap::new();
    map.insert(1,2);    
}

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// // --snip--
// use std::cmp::Ordering;
// use std::io;
// // --snip--

// // --snip--
// use std::{cmp::Ordering, io};
// // --snip--

// use std::io;
// use std::io::Write;

use std::io::{self, Write};
use std::collections::*;
