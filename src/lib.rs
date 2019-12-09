// // Setup error chain

// #![recursion_limit = "1024"]
// #[macro_use]
// extern crate error_chain;

// // We'll put our errors in an `errors` module, and other modules in
// // this crate will `use errors::*;` to get access to everything
// // `error_chain!` creates.
// pub mod errors {
//     // Create the Error, ErrorKind, ResultExt, and Result types
//     error_chain! { }
// }


mod options;
pub mod grid;
pub mod intcode;
pub mod io;
pub use options::StandardOptions;