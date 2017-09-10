extern crate unterflow;
extern crate tokio_core;

use tokio_core::reactor::Core;

use unterflow::Client;

fn client(core: &mut Core) -> Client {
    let addr = "127.0.0.1:51015".parse().unwrap();

    let client = Client::connect(&addr, &core.handle());
    core.run(client).unwrap()
}

#[test]
fn test_topology() {
    let mut core = Core::new().unwrap();
    let client = client(&mut core);

    let topology = core.run(client.topology()).unwrap();

    assert_eq!(1, topology.brokers.len());

    let broker = &topology.brokers[0];

    assert_eq!(
        Some(1),
        topology.topic_leaders.get("default-topic").map(|topic| {
            topic.len()
        })
    );
    assert_eq!(
        Some(broker),
        topology.topic_leaders.get("default-topic").and_then(
            |topic| {
                topic.get(&0)
            },
        )
    );
    assert_eq!(
        Some(1),
        topology.topic_leaders.get("internal-system").map(|topic| {
            topic.len()
        })
    );
    assert_eq!(
        Some(broker),
        topology.topic_leaders.get("internal-system").and_then(
            |topic| {
                topic.get(&0)
            },
        )
    );
}

#[test]
fn test_create_task() {
    let mut core = Core::new().unwrap();
    let client = client(&mut core);

    let task = client.new_task("foo".to_string()).retires(12).add_header(
        "foo".to_string(),
        "bar".to_string(),
    );

    let task = core.run(task.create("default-topic")).unwrap();

    assert_eq!("CREATED", task.state);
    assert!(task.key > 0);
    assert!(task.key < u64::max_value());
}
