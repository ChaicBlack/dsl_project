mod conn;
mod message;
mod node;

fn main() {
    let node = node::Node::new(1, "Node1".to_string(), "node1.log");

    node.start();
    node.do_something();

    if let Err(e) = node.send_message("Hello, world!", "127.0.0.1:8080") {
        eprintln!("Failed to send message: {}", e);
    }

    node.stop();

    log::logger().flush();
}
