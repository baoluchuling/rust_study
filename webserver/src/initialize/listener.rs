
use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};

use crate::{Route, StringExtension};

static _ROOT_PATH: &str = "/Users/admin/Desktop/rust_study/webserver/src";

pub struct Listener {
    route: Route
}

impl Listener {
    pub fn new(route: Route) -> Listener {
        Listener {
            route
        }
    }

    pub fn run(&mut self) {
            // let (mut tcp_stream, addr) = listener.accept()?; // 阻塞，直到被请求
            // println!("Connection received! {:?} is sending data.", addr);
            // let mut input = String::new();
            // let _ = tcp_stream.read_to_string(&mut input)?;
            // println!("{:?} says {}", addr, input);
            // Ok(())

        let listener:TcpListener = TcpListener::bind("127.0.0.1:55273").unwrap();
        let port = listener.local_addr();
        println!("Listening on {}, access this port to end the program", port.unwrap());

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.handle_connection(stream);
        }
    }

    // 接口监听回调
    fn handle_connection(&mut self, mut stream: TcpStream) {

        let request_infos: Vec<String> = Self::parse_request(&stream);

        println!("Request Info: \n {}", request_infos.join("\n"));
    
        let res: Vec<&str> = request_infos.first().unwrap().split(' ').collect();
    
        let _method = res[0];
        let _path = res[1];
        let _version = res[2];
    
        let mut _response = "".to_string();
        if _method == "GET" {
            _response = self.route.invoke(_path.to_string());
        }
    
        // 将回复内容写入连接缓存中
        stream.write_all(_response.as_bytes()).unwrap();
        // 使用flush将缓存中的内容发送到客户端
        stream.flush().unwrap();
    }
    
    // 请求信息初步处理
    fn parse_request(mut request: &TcpStream) -> Vec<String> {
        // 从连接中顺序读取 1024 字节数据
        let mut buffer = [0; 1024];
        request.read(&mut buffer).unwrap();
    
        let binding = String::from_utf8_lossy(&buffer);
    
        return StringExtension::split(binding.to_string(), "\n");
    }
}