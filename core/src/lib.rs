pub use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub use brotli;
pub use flate2;
pub use http_scrap::{HMap, QMap, Response};
use threadpool::GatesThread;
pub use zstd;
// use tokio_tungstenite::tungstenite::WebSocket;
mod threadpool;
pub use gate::gates;
pub use rusty_format::cors::Cors;

#[derive(Debug)]
pub struct GatesResponse {
    pub header: Vec<String>,
    pub status: i32,
    pub message: String,
    pub content_type: Option<String>,
    pub encoding: Option<String>,
}

impl GatesResponse {
    pub fn new() -> Self {
        Self {
            header: Vec::new(),
            message: String::new(),
            status: 200,
            content_type: Some("application/json".to_string()),
            encoding: None,
        }
    }
    pub fn status(mut self, code: i32) -> Self {
        self.status = code;
        self
    }
    pub fn content_type(mut self, content: impl Into<String>) -> Self {
        let content: String = content.into();
        let conten = match content.as_str() {
            "json" => "application/json",
            "text" => "text/plain",
            "html" => "text/html",
            "form-data" => "application/x-www-form-urlencoded",
            "multi-form-data" => "mutipart/form-data",
            "javascript" => "application/javascript",
            "css" => "text/css",
            "xml" => "application/xml",
            "csv" => "text/csv",
            "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "mp4" => "video/mp4",
            "webm" => "video/webm",
            "pdf" => "application/pdf",
            _ => "text/plain",
        };
        self.content_type = Some(conten.to_string());
        self
    }
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let header = format!("{}: {}", key.into(), value.into());
        self.header.push(header);
        self
    }
    pub fn encoding_type(mut self, value: impl Into<String>) -> Self {
        self.encoding = Some(value.into());
        self
    }
    pub fn message<'great>(&'great mut self, message: impl Into<String>) -> Return {
        self.message = message.into();

        Return {
            status: self.status,
            content_type: self.content_type.clone(),
            headers: Some(self.header.clone()),
            message: self.message.clone(),
            encoding: self.encoding.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Return {
    pub status: i32,
    pub message: String,
    pub content_type: Option<String>,
    pub headers: Option<Vec<String>>,
    pub encoding: Option<String>,
}

pub enum Middleware<'b> {
    Cors(&'b str),
    Compression(&'b str),
    CustomeHeader(&'b str, &'b str),
    // Jwt(bool),
}

#[derive(Debug)]
pub struct GatesRequest<'req> {
    pub method: &'req str,
    pub path: &'req str,
    pub headers: HMap<&'req str, &'req str>,
    pub query: QMap<&'req str, &'req str>,
    pub param: HashMap<String, String>,
    pub body: &'req str,
    pub cookies: &'req str,
}

#[derive(Debug)]
pub enum GateKeeperResponse {
    Next,
    Response(Return),
    Redirect(u64, &'static str),
}

pub struct GateKeeper;
impl GateKeeper {
    pub fn next() -> GateKeeperResponse {
        GateKeeperResponse::Next
    }
    pub fn response(response: Return) -> GateKeeperResponse {
        GateKeeperResponse::Response(response)
    }
    pub fn redirect(status: u64, redirect: &'static str) -> GateKeeperResponse {
        GateKeeperResponse::Redirect(status, redirect)
    }
}

pub struct GateResponse;
impl GateResponse {
    pub fn response(response: Return) -> GateKeeperResponse {
        GateKeeperResponse::Response(response)
    }
    pub fn redirect(status: u64, redirect: &'static str) -> GateKeeperResponse {
        GateKeeperResponse::Redirect(status, redirect)
    }
}
// pub enum GatesResponse {
//     Next,
//     Response(Return),
//     Redirect(u64, &'static str),
// }

// pub struct Ws<'b> {
// pub ws: WebSocket<&'b TcpStream>,
// }

type Buck = fn(&TcpStream, &Response, &str, &str, Vec<(&&str, &&str)>);
// type WsBuck = fn(&TcpStream, &Response, &WebSocket<&TcpStream>);

pub struct Gates<'billionaire> {
    port: String,
    middleware: Vec<Middleware<'billionaire>>,
    routes: Vec<Buck>,
    // ws_routes: Vec<WsBuck>,
}

impl<'gates> Gates<'gates> {
    pub fn new() -> Self {
        Self {
            port: String::new(),
            routes: Vec::new(),
            middleware: Vec::new(),
            // ws_routes: Vec::new(),
        }
    }
    pub fn port(mut self, port: impl Into<String>) -> Self {
        self.port = port.into();
        self
    }
    pub fn routes(mut self, routes: &[Buck]) -> Self {
        self.routes = routes.to_vec();
        self
    }
    // pub fn ws_routes(mut self, ws: &[WsBuck]) -> Self {
    //     self.ws_routes = ws.to_vec();
    //     self
    // }
    pub fn middleware(mut self, middleware: Middleware<'gates>) -> Self {
        self.middleware.push(middleware);
        self
    }
    pub fn run(self) {
        let listener = TcpListener::bind(&self.port).unwrap();
        let gates_threads = GatesThread::new(4);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            // let stream = Arc::new(RwLock::new(stream));
            // let b = stream.try_clone().unwrap();
            let value = self.handle_connection(&stream);
            gates_threads.execute(move || {
                value;
            });
        }
    }
    fn handle_connection(&self, mut stream: &TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer);

        let response = String::from_utf8_lossy(&buffer);
        // println!("{}", response)
        // if response.starts_with("OPTIONS") {
        //     let b = format!("HTTP/1.1 204 No Content\r\n{}", b);
        //     println!("{}", b);
        //     println!("{}", "billionaire");
        //     stream.write_all(b.as_bytes());
        //     stream.flush();
        // } else {
        let mut cp = &"";
        let mut cors_header = String::new();

        // let mut stream = &*stream.write().unwrap();
        let mut custome_headers = Vec::new();
        for middleware in self.middleware.iter() {
            match middleware {
                Middleware::Cors(cors) => {
                    let b = format!(
                        "HTTP/1.1 204 No Content\r\n{}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                        cors
                    );
                    cors_header.push_str(cors);
                    if response.starts_with("OPTIONS") {
                        // println!("{}", b);
                        stream.write_all(b.as_bytes());
                        stream.flush();
                    }
                }
                Middleware::Compression(types) => {
                    cp = types;
                }
                Middleware::CustomeHeader(header, value) => {
                    custome_headers.push((header, value));
                }
            }
            // // println!("{}", b);
            // // println!("{}", "billionaire");
            // stream.write_all(b.as_bytes());
            // stream.flush();
        }

        let response = Response::new(&buffer);
        for route in self.routes.iter() {
            route(stream, &response, cp, &cors_header, custome_headers.clone());
        }
    }
}
