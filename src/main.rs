//---   Imports   ---//
use rouille::{ router, Response, try_or_400, post_input };
// use serde_json::{ from_str, Value};

//---   Global Variables   ---//
const LISTEN_ON: &str = "0.0.0.0:9988";

//---   Start Server   ---//
fn main() {
    println!("FirewallD-http is listening at: {}", LISTEN_ON);
    println!("!!!   IMPORTANT NOTICE   !!!");
    println!("Ensure that this port is only able to accessed by trusted applications");
    println!("Any program or user that can access {} is able to change your firewall configuration.", LISTEN_ON);
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    rouille::start_server(LISTEN_ON, move |request| {
        router!(request,
            (POST) (/) => {
                let raw_data = try_or_400!(post_input!(request, {
                    firstName: String,
                    lastName: String,
                }));
                println!("{:?}", raw_data);
                Response::text("hello world")
            },
            (GET) (/) => {
                Response::text("You sent a 'GET' request. To use FirewallD-http, please make a 'POST' request.")
            },
            _ => rouille::Response::empty_404()
        )
    });
}