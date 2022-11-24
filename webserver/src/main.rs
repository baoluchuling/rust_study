mod initialize;
mod utils;

use std::fs;

pub use initialize::{listener::Listener, route::Route};
pub use utils::string::StringExtension;

fn main() {

    let _route_map = StringExtension::split(fs::read_to_string("/Users/admin/Desktop/rust_study/webserver/src/assets/route.list").unwrap(), "\n");

    let route = Route::new(path);

    let mut listener: Listener = Listener::new(route);

    listener.run();
}
