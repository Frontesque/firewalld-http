//---   Imports   ---//
use rouille::{ router, Response };

//---   Global Variables   ---//
const LISTEN_ON: &str = "0.0.0.0:9988";

//---   Start Server   ---//
fn main() {
    rouille::start_server(LISTEN_ON, move |request| {
        router!(request,
            (POST) (/) => {
                Response::text("hello world")
            },
            (GET) (/) => {
                Response::text("You sent a 'GET' request. To use FirewallD-http, please make a 'POST' request.")
            },
            _ => rouille::Response::empty_404()
        )
    });
}