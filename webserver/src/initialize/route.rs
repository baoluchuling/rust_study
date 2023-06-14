use std::{fs};

use crate::keys::ROOT_PATH;

use crate::StringExtension;
use serde::{Serialize, Deserialize};

pub struct Route {
    map: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    a: i32,
    b: i32,
}

impl Route {
    pub fn new(map: Vec<String>) -> Route {
        Route {
            map
        }
    }

    pub fn invoke(&mut self, path: String) -> String {
        let mut _status_line = "";
        let mut _contents = "".to_string();

        let paths = StringExtension::split(path.to_string(), "/?");

        let _method = &paths[0];
        let _param_str = if paths.len() > 1 { &paths[1] } else { "{}"};

        print!("{} {}", _method, _param_str);
        
        if self.map.contains(_method) {
            if _method == "/" {
                _status_line = "HTTP/1.1 200 OK\r\n\r\n";
                _contents = fs::read_to_string(ROOT_PATH.to_string() + &"/assets/hello.html").unwrap();
            } else if _method == "/version/" || _method == "/version" {
                _status_line = "HTTP/1.1 200 OK\r\n\r\n";
                _contents = "{\"code\": 200, \"message\": \"1.0.0\"}".to_string();
            } else if _method == "/plus/" || _method == "/plus" {
                let _param_list_str = StringExtension::split(_param_str.to_string(), "&");
        
                let mut json = "".to_string();
                for _ps in _param_list_str {
                    let _kv = StringExtension::split(_ps.to_string(), "=");
                    let key = _kv[0].to_string();
                    let value = _kv[1].to_string();
                    json = (if json.is_empty() { "".to_string() } else { json.to_string() }) + (if json.is_empty() { "" } else { "," }) + "\"" + &key + "\":" + &value + "";
                }
                json = "{".to_string() + &json + "}";
        
                let deserialized: Point = serde_json::from_str(&json).unwrap();
                
                _status_line = "HTTP/1.1 200 OK\r\n\r\n";
                _contents = ("{\"code\": 200, \"data\": ".to_string() + &(deserialized.a + deserialized.b).to_string() + ", \"message\": \"ok\"}").to_string();
            }
        } else {
            _status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            _contents = fs::read_to_string(ROOT_PATH.to_string() + &"/assets/404.html").unwrap();
        }
        
        return format!("{_status_line}{_contents}");
    }
}