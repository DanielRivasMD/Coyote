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

// TODO: consider changing to static
const DATE_FORMAT: &str = "%Y-%m-%d";
const TRAIN_SUCCESS: [&str; 3] = ["Suberb!", "Outstanding!", "Remarkable!"];
const TRAIN_FAILURE: [&str; 3] = ["Keep it up!", "Do not give up just yet!", "Next time it will be better!"];

////////////////////////////////////////////////////////////////////////////////////////////////////
