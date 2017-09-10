extern crate unterflow;
extern crate tokio_core;
extern crate futures;

use futures::future;
use tokio_core::reactor::Core;

use unterflow::Client;

const REQUESTS: usize = 1_000_000;

fn main() {
    let mut core = Core::new().unwrap();
    let addr = "127.0.0.1:51015".parse().unwrap();

    let client = Client::connect(&addr, &core.handle());
    let client = core.run(client).unwrap();

    let topic = "default-topic";
    let task = client.new_task("foo".to_string()).retires(12).add_header(
        "foo".to_string(),
        "bar".to_string(),
    );


    let futures: Vec<_> = (0..REQUESTS).map(|_| task.create(topic)).collect();
    let results = core.run(future::join_all(futures)).unwrap();

    let results = results.len();
    assert_eq!(REQUESTS, results)
}
