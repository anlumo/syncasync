use tokio::prelude::*;
use tokio::timer::Delay;
use std::time::{Duration, Instant};

struct Foo {}

trait Sync {
    fn wait(&self);
}

trait Async {
    type Return: Future<Item = (), Error = tokio::timer::Error>;
    fn wait(&self) -> Self::Return;
}

impl Sync for Foo {
    fn wait(&self) {
        std::thread::sleep(Duration::from_millis(1000));
    }
}

impl Async for Foo {
    type Return = Delay;
    fn wait(&self) -> Delay {
        Delay::new(Instant::now() + Duration::from_millis(1000))
    }
}

fn main() {
    let foo = Foo {};
    println!("Sync");
    <Foo as Sync>::wait(&foo);
    println!("Async");
    tokio::run(<Foo as Async>::wait(&foo).map_err(|_| {}));
}
