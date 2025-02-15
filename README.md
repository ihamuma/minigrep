# minigrep

## Introduction

**minigrep** is a command line [grep](https://en.wikipedia.org/wiki/Grep) tool written in Rust. It is based on the I/O Project of Chapter 12 in [The Rust Programming Language](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html).

## Running minigrep

minigrep is not yet, and likely never will be, a crate on [crates.io](https://crates.io/). As such, to run minigrep, fork or clone this repository on your machine. Prerequisites include [rustup](https://rustup.rs/) and [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

With the repository cloned, simply navigate to it in a terminal window and run it with cargo run. minigrep takes three command line arguments: the string to search for, the filepath and a boolean indicating case sensitivity.