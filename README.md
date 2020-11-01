# Overview  
  
This repository is a playground (hence the name) in which I try to experiment with various compression algorithms.
Each algorithm will be implemented in it's own folder which will contain a README file in which I will try my best to describe the Mathematics behind the algorithm as well as how it works.
The code will be written in Rust because it's a language I love and I'm trying to get better at.
At the moment every folder will be a separate crate but a binary one. If I feel confident for my work in the future, I might turn these crates into library ones and upload them to Rust package registry for public use.
Finally, I will try my best to keep the code documented and provide unit tests for it as well as a main file that will provide a command line program to test the algorithms.

# How to run the algorithms
First of all you will need to clone this repository so you can get the source code locally.
Then, since we are using Rust, it will be pretty easy to compile and run these algorithms.
Just `cd` into the relative folder and run `cargo build` for a dev build or `cargo build --release` for a release build.
You can then run the program using `cargo run -q` or `cargo -q --release` for dev and release run respectively (`-q` is for quite so cargo doesn't print some unnecessary build information).
However, my programs will most probably require command line arguments.
In order to pass command line arguments run `cargo -q -- [program options]`.
To see **program_options** run `cargo -q -- --help`.