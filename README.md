example.rs
==========

Rusty experiments.
Basically this repo is beeing used to get used to `rustpkg`and `rust`.


Build
-----
The intention is that this example can be build using `rustpkg`::

    $ rustpkg build helloworld


Run
---
Alternatively `rust` can be used to directly run the code:

    $ rust run src/helloworld/main.rs

The `RUST_LOG` environment variable controls the output of the
[logging system](http://static.rust-lang.org/doc/rust.html#logging-system):

    $ RUST_LOG=main=4 src/helloworld/main.rs
