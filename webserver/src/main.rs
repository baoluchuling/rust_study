mod initialize;
mod keys;
mod utils;

use std::fs;

use crate::keys::ROOT_PATH;

pub use initialize::{listener::Listener, route::Route};
pub use utils::string::StringExtension;

fn main() {
    // get route list
    let result = fs::read_to_string(ROOT_PATH.to_string() + "/assets/route.list");
    if result.is_err() {
        println!("Error: {}", result.err().unwrap());
        return;
    }

    let _route_map = StringExtension::split(
        fs::read_to_string(ROOT_PATH.to_string() + "/assets/route.list").unwrap(),
        "\n",
    );

    let route = Route::new(_route_map);

    let mut listener: Listener = Listener::new(route);

    listener.run();
}
