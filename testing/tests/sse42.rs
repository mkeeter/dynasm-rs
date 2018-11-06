#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]

#[macro_use]
extern crate dynasmrt;
extern crate dynasm;

use dynasm::dynasm;
use dynasmrt::DynasmApi;

include!("gen/sse42.rs.gen");
