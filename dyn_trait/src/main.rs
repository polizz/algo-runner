use rand::random;

trait DoIt {
  fn doit(&self) -> String;
}

#[derive(Debug)]
struct Person;
#[derive(Debug)]
struct Dog;

impl DoIt for Person {
  fn doit(&self) -> String {
    format!("I'm a person")
  }
}

impl DoIt for Dog {
  fn doit(&self) -> String {
    format!("I'm a dog")
  }
}

fn tester() -> Box<dyn DoIt> {
  let rand_branch = random::<bool>();

  let whatever: Box<dyn DoIt> = if rand_branch {
    let p = Person {};
    Box::from(p)
  } else {
    Box::new(Dog {})
  };

  whatever
}

fn main() {
  let ret = tester();
  let st = ret.doit();

  println!("Heap DoIt was: {}", &st);

  let stack_whatever: &dyn DoIt = if random::<bool>() {
    &Person {}
  } else {
    &Dog {}
  };

  println!("Stack DoIt was: {}", &stack_whatever.doit());
}
