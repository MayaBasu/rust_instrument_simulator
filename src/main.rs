#![allow(warnings)]
mod objects;
use bytes::Buf;
use serde_yaml;
mod simulator_engine;
mod uvex;
use rayon::prelude::*;
mod hallucinate;
mod effects;
mod instrument;


fn main() {

    simulator_engine::pipline(true)
}


