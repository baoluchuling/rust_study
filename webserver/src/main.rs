mod initialize;
mod utils;
mod keys;

use std::fs;

use crate::keys::ROOT_PATH;

pub use initialize::{listener::Listener, route::Route};
pub use utils::string::StringExtension;

fn main() {
    let path = [
        "/".to_string(),
        "/plus".to_string(),
        "/?version".to_string()
    ].to_vec();

    let _route_map = StringExtension::split(fs::read_to_string(ROOT_PATH.to_string() + "/assets/route.list").unwrap(), "\n");

    let route = Route::new(path);

    let mut listener: Listener = Listener::new(route);

    listener.run();
}