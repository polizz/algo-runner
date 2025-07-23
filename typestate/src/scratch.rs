#![allow(dead_code)]
use std::marker::PhantomData;

struct Init;
struct Mode1;
struct Mode2;

// Make marking structs suitable for use as Modes
impl Mode for Init {}
impl Mode for Mode1 {}
impl Mode for Mode2 {}

trait Mode {}

struct BuilderStruct<M: Mode> {
  ident: usize,
  marker: PhantomData<M>,
}

impl<M: Mode> BuilderStruct<M> {
  fn new(ident: usize) -> BuilderStruct<Init> {
    BuilderStruct::<Init> {
      ident,
      marker: Default::default(),
    }
  }
  fn ident(&self) -> String {
    format!("Ident: {}", &self.ident)
  }
  fn set_mode1(self) -> BuilderStruct<Mode1> {
    BuilderStruct::<Mode1> {
      ident: self.ident,
      marker: Default::default(),
    }
  }
  fn set_mode2(self) -> BuilderStruct<Mode2> {
    BuilderStruct::<Mode2> {
      ident: self.ident,
      marker: Default::default(),
    }
  }
}

impl BuilderStruct<Mode1> {
  fn output_mode1(&self) -> String {
    format!("Mode1 Ident: {}", &self.ident)
  }
}

impl BuilderStruct<Mode2> {
  fn output_mode2(&self) -> String {
    format!("Mode2 Ident: {}", &self.ident)
  }
}

pub fn run() {
  let b1 = BuilderStruct::<Init>::new(1);

  println!("Ident for b1 is: {}", b1.ident());

  println!(
    "Ident for b1 after set_mode2 is: {}",
    b1.set_mode2().output_mode2()
  );
}
