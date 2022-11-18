#![cfg(fuzzing)]

#[macro_use]
extern crate gfx;
pub use gfx_voxel::{array, cube};

pub mod chunk;
pub mod minecraft;
pub mod shader;
