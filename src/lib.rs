// [[file:../nauty.note::*include][include:1]]
#![allow(nonstandard_style)]

#[allow(clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub(crate) use bindings::*;
// include:1 ends here

// [[file:../nauty.note::*mods][mods:1]]
mod nauty;
// mods:1 ends here
