//! Manages commands related to subjects.

use diesel::{RunQueryDsl};

mod add;
pub mod interpreter;
mod list;
mod mark;
mod modify;
mod remove;
mod usage;