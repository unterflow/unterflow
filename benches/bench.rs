#![feature(test)]

extern crate test;

use test::Bencher;

extern crate unterflow;
extern crate tokio_core;
extern crate futures;

use futures::future;
use tokio_core::reactor::Core;

use unterflow::Client;

const REQUESTS: usize = 1_000;

#[bench]
fn topology(b: &mut Bencher) {
    let mut core = Core::new().unwrap();
    let client = client(&mut core);

    b.iter(|| {
        let futures: Vec<_> = (0..REQUESTS).map(|_| client.topology()).collect();
        core.run(future::join_all(futures)).unwrap();
    });
}

#[bench]
fn task(b: &mut Bencher) {
    let mut core = Core::new().unwrap();
    let client = client(&mut core);

    let topic = "default-topic";
    let task = client.new_task("foo".to_string()).retires(12).add_header(
        "foo".to_string(),
        "bar".to_string(),
    );


    b.iter(|| {
        let futures: Vec<_> = (0..REQUESTS).map(|_| task.create(topic)).collect();
        core.run(future::join_all(futures)).unwrap();
    });
}

fn client(core: &mut Core) -> Client {
    let addr = "127.0.0.1:51015".parse().unwrap();

    let client = Client::connect(&addr, &core.handle());
    core.run(client).unwrap()
}
