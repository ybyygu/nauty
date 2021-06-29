A simple rust wrapper for [nauty](http://users.cecs.anu.edu.au/~bdm/nauty/) lib, so that can be statically linked using
[MUSL](https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html).

For now, this crate mainly used for canonical graph labelling such that any two
isomorphic graphs have the same canonical node labelling.

