# core-io [![Build Status](https://travis-ci.org/QuiltOS/core-io.svg?branch=master)](https://travis-ci.org/QuiltOS/core-io)

`core-io` is an alternative `no_std` I/O library for Rust. 
Unlike `std::io`, it uses an associated error type instead of baking in a single `io::Error`.
This way it supports a wider variety of implementees and only depend on `core`.
Other than that difference, it seeks to match `std::io`'s traits as closely as possible. 
