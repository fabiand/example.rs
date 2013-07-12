example.rs
==========

Rusty experiments.

Commands
--------
`rustc` - The rust compiler, most low-level command
`rust` - A convenience tool to handle source *files*
`rustpkg` - A package management tool to build *packages*

Build
-----
The intention is that this example can be build using `rustpkg`::

    $ git clone https://github.com/fabiand/example.rs example
    $ cd example
    $ rustpkg build rustyworld

or using `rustpkg`'s own instal mechanism:

   $ rustpkg install github.com/fabiand/example.rs

Run
---
Alternatively `rust` can be used to directly run the code:

    $ rust run src/rustyworld/main.rs

Debugging
---------
The `RUST_LOG` environment variable controls the output of the
[logging system](http://static.rust-lang.org/doc/rust.html#logging-system):

    $ RUST_LOG=main=4 src/rustyworld/main.rs

