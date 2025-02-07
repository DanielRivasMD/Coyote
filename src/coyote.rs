////////////////////////////////////////////////////////////////////////////////////////////////////

// set compiler lints
#![allow(unused_variables)]

////////////////////////////////////////////////////////////////////////////////////////////////////
// macros
////////////////////////////////////////////////////////////////////////////////////////////////////

// derive new
#[macro_use]
extern crate derive_new;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
pub mod cmds;
pub mod custom;
pub mod utils;

////////////////////////////////////////////////////////////////////////////////////////////////////

const DATE_FORMAT: &str = "%Y-%m-%d";

////////////////////////////////////////////////////////////////////////////////////////////////////
