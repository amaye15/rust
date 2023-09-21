mod client;

use crate::client::Service;

fn main() {
    let mut service = client::Service::new("https://splunk-instance-url");
    service.set_auth_token("your_auth_token");
    
    let my_index = service.index("my_index");
    match my_index.submit("some event", "myevent") {
        Ok(_) => println!("Event submitted successfully!"),
        Err(e) => eprintln!("Error submitting event: {}", e),
    }
}
