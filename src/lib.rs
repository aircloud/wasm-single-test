#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;
extern crate serde_json;

use web_sys;

use std::{sync::Mutex};
use wasm_bindgen::prelude::*;
use std::sync::PoisonError;
use std::panic;

pub struct CurrentStatus {
  pub index: i32,
}

impl CurrentStatus {
  fn new() -> Self {
    CurrentStatus {
      index: 1,
    }
  }
  fn get_index(&mut self) -> i32 {
    self.index.clone()
  }

  fn add_index(&mut self) {
    self.index += 1;
  }
}

lazy_static! {
    pub static ref FOO: Mutex<CurrentStatus> = Mutex::new(CurrentStatus::new());
}

#[wasm_bindgen]
pub fn add_index() {
  FOO.lock().unwrap_or_else(PoisonError::into_inner).add_index();
}

#[wasm_bindgen]
pub fn get_index() -> i32 {
  let mut foo = FOO.lock().unwrap_or_else(PoisonError::into_inner);
  return foo.get_index();
}
