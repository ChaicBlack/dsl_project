mod conn;
mod message;
mod node;

fn main() {
    let node = node::Node::new(
        1,
        "Node1".to_string(),
        "node1.log",
        "127.0.0.1:8080",
        vec!["127.0.0.1:8081", "127.0.0.1:8082"],
    );

    node.start();
    node.do_something();
    node.stop();

    log::logger().flush();
}
