use pandas_pouch::client::Client;
use std::time::Duration;

fn main() {
    let mut client = Client::new("localhost", 11211, 300, Some(Duration::from_secs(5)));

    client.put("key1", "value1");
    let value = client.get("key1");
    println!("{:?}", value);
}
