// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(warnings)]
#![feature(in_band_lifetimes)]

fn foo(x: &'a u32, y: &u32) -> &'a u32 { y } //~ ERROR explicit lifetime required

fn foo2(x: &'a u32, y: &'b u32) -> &'a u32 { y } //~ ERROR lifetime mismatch

fn main() {}
