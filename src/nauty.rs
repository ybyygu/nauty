// [[file:../nauty.note::*imports][imports:1]]
use crate::*;

use anyhow::Result;
use std::os::raw::c_int;
// imports:1 ends here

// [[file:../nauty.note::*default][default:1]]
// bindgen doesn't get the types right for the following constants
pub const FALSE: boolean = bindings::FALSE as boolean;
pub const TRUE: boolean = bindings::TRUE as boolean;
pub const CONSOLWIDTH: c_int = bindings::CONSOLWIDTH as c_int;

impl std::default::Default for optionblk {
    fn default() -> Self {
        optionblk {
            getcanon: 0,
            digraph: FALSE,
            writeautoms: FALSE,
            writemarkers: FALSE,
            defaultptn: TRUE,
            cartesian: FALSE,
            linelength: CONSOLWIDTH,
            outfile: std::ptr::null_mut(),
            userrefproc: None,
            userautomproc: None,
            userlevelproc: None,
            usernodeproc: None,
            usercanonproc: None,
            invarproc: None,
            tc_level: 100,
            mininvarlevel: 0,
            maxinvarlevel: 1,
            invararg: 0,
            dispatch: unsafe { &mut dispatch_graph },
            schreier: FALSE,
            extra_options: std::ptr::null_mut(),
        }
    }
}
// default:1 ends here

// [[file:../nauty.note::*test][test:1]]
#[cfg(test)]
mod tests {
    use super::*;
    use ::std::os::raw::c_int;

    #[test]
    fn nautyex2() {
        let mut options = optionblk::default();
        // Select option for canonical labelling
        options.getcanon = TRUE;
        options.defaultptn = FALSE;
    }
}
// test:1 ends here
