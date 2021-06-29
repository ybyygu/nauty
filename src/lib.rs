// [[file:../nauty.note::*include][include:1]]
#![allow(nonstandard_style)]

#[allow(clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub(crate) use bindings::*;
// include:1 ends here

// [[file:../nauty.note::*imports][imports:1]]
use std::os::raw::c_int;
// imports:1 ends here

// [[file:../nauty.note::*mods][mods:1]]
mod nauty;
// mods:1 ends here

// [[file:../nauty.note::*pub][pub:1]]
#[cfg(feature = "adhoc")]
pub use bindings::dwim;
// pub:1 ends here

// [[file:../nauty.note::*test][test:1]]
#[test]
fn test_nauty() {
    let n = 4;
    let mut ptn = vec![0; n];
    let mut lab = vec![0; n];
    let mut edges_i = vec![0, 0, 1];
    let mut edges_j = vec![1, 2, 3];
    let nnodes = n as i32;
    let nedges = 3 as i32;
    unsafe {
        let x = dwim(
            nnodes,
            nedges,
            edges_i.as_mut_ptr(),
            edges_j.as_mut_ptr(),
            lab.as_mut_ptr(),
            ptn.as_mut_ptr(),
        );
        dbg!(x);
        dbg!(lab);
        dbg!(ptn);

    }
}
// test:1 ends here
