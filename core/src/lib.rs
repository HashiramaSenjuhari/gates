pub use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
use std::{
    io::{BufReader, Cursor},
    thread,
};

pub use brotli;
pub use flate2;
pub use http_scrap::{HMap, QMap, Response};
use rusty_rl::FixedLimit;
use sergy::page;
use threadpool::GatesThread;
pub use tokio_tungstenite::tungstenite::{Message, WebSocket, accept};
pub use zstd;
mod threadpool;
pub use rusty_format::cors::Cors;
pub use rusty_gate::gates;
pub use rusty_gatesdope::gates_dope;

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
    CustomeHeader(&'b str, &'b str), // Jwt(bool),
}

pub enum RL<'b> {
    /// takes ## Total Request and Interval
    FixedLimit(usize, u64, &'b str), // FixedBucket()
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
type WsBuck = fn(&mut GatesDope, String);

pub enum Static<'b> {
    All,
    Pages(&'b [&'b str]),
    CustomePage(&'b [&'b str], &'b str),
    AllAndCustome404(&'b str),
}
pub struct Gates<'billionaire> {
    port: String,
    middleware: Vec<Middleware<'billionaire>>,
    routes: Vec<Buck>,
    ws_routes: Vec<WsBuck>,
    statics: (String, Static<'billionaire>),
    rl: Vec<Option<RL<'billionaire>>>,
}

pub struct PrependBuffer<B> {
    prepend_buffer: Cursor<Vec<u8>>,
    inner: B,
}

pub type GatesDope<'billionaire> = WebSocket<PrependBuffer<&'billionaire TcpStream>>;

impl<'gates> Gates<'gates> {
    pub fn new() -> Self {
        Self {
            port: String::new(),
            routes: Vec::new(),
            middleware: Vec::new(),
            ws_routes: Vec::new(),
            statics: (String::new(), Static::All),
            rl: Vec::new(),
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
    pub fn ws_routes(mut self, routes: &[WsBuck]) -> Self {
        self.ws_routes = routes.to_vec();
        self
    }
    pub fn statics(mut self, dir: impl Into<String>, file: Static<'gates>) -> Self {
        self.statics = (dir.into(), file);
        self
    }
    pub fn rate_limit(mut self, kind: RL<'gates>) -> Self {
        self.rl.push(Some(kind));
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
        let mut s: Vec<(Option<FixedLimit>, &str)> = Vec::new();
        for b in self.rl.iter() {
            if let Some(b) = b {
                match b {
                    RL::FixedLimit(maximum, interval, route) => {
                        let allowed = FixedLimit::new().maximum_request(*maximum, *interval);
                        s.push((Some(allowed), route));
                    }
                }
            }
        }
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let value = self.handle_connection(&stream, &mut s);
            gates_threads.execute(move || {
                value;
            });
            // let stream = Arc::new(RwLock::new(stream));
            // let b = stream.try_clone().unwrap();
            // println!("{}", "billionairegreathari");
        }
    }
    fn handle_connection(
        &self,
        mut stream: &TcpStream,
        ratelimit: &mut Vec<(Option<FixedLimit>, &str)>,
    ) {
        let mut buffer = [0; 1024];
        let mut string = String::new();
        let b = stream.read(&mut buffer).unwrap();
        // println!("{}", string);

        let read_response = String::from_utf8_lossy(&buffer);
        let response = Response::new(&read_response.to_string());
        let path = response.path();

        for b in ratelimit {
            match b {
                (Some(b), route) => {
                    // println!("{}", route);
                    if *route == path {
                        // println!("{}", "hyguiokjyiiuoiopii0op");
                        let allowed = b.allow(stream.peer_addr().unwrap().ip().to_string(), *route);
                        let page = format!(
                            "HTTP/1.1 429 Too Many Request\r\nContent-Length: {}\r\n\r\n{}",
                            "billionairegreathari".len(),
                            "billionairegreathari"
                        );
                        let is_allowed = allowed.is_allowed;
                        // println!("{}", is_allowed);
                        if !is_allowed {
                            // println!("{}", page);
                            stream.write_all(page.as_bytes());
                            stream.flush();
                            return;
                        }
                    } else if route.contains("/*") {
                        // println!("{}", route);
                        let bi = route.replace("*", "");
                        // println!("{}", bi);
                        // println!("{}", path);
                        if path.starts_with(&bi) {
                            let allowed =
                                b.allow(stream.peer_addr().unwrap().ip().to_string(), *route);
                            // println!("{}", allowed.remaning_count);
                            if !allowed.is_allowed {
                                let page = format!(
                                    "HTTP/1.1 429 Too Many Request\r\nContent-Length: {}\r\n\r\n{}",
                                    "billionaires".len(),
                                    "billionaires"
                                );
                                stream.write_all(page.as_bytes());
                                stream.flush();
                            }
                        }
                    } else if *route == "*" {
                        // println!("{}", "________________________________________________");
                        let allowed = b.allow(stream.peer_addr().unwrap().ip().to_string(), *route);
                        let page = format!(
                            "HTTP/1.1 429 Too Many Request\r\nContent-Length: {}\r\n\r\n{}",
                            "billionairegreathari".len(),
                            "billionairegreathari"
                        );
                        let is_allowed = allowed.is_allowed;
                        if !is_allowed {
                            // println!("{}", page);
                            stream.write_all(page.as_bytes());
                            stream.flush();
                            return;
                        }
                    }
                    // else if *route != "*" && route.contains("/*") && !is_allowed {
                    // let b = route.replace("*", "");
                    // if path.starts_with(&b) {}
                    // }
                }
                _ => {}
            }
        }
        // println!(
        //     "======================================================{}",
        //     "billionaire"
        // );
        let mut cp = &"";
        let mut cors_header = String::new();
        // let is_allowed: bool = true;

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
                    if read_response.starts_with("OPTIONS") {
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

        // match is_allowed {
        //     true => {

        if !path.starts_with("/api") && !path.starts_with("/ws") {
            let bmode = &self.statics.1;
            match bmode {
                Static::All => {
                    let html = format!("{}{}/server.html", self.statics.0, path);
                    // println!("{}", html);
                    let b = page!(self.statics.0, html);
                    // println!("{}", b.as_ref().unwrap());
                    match b {
                        Some(b) => {
                            let b = format!(
                                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                                b.len(),
                                b
                            );
                            // println!("{}", b);
                            stream.write_all(b.as_bytes());
                            stream.flush();
                        }
                        None => {
                            let b = format!(
                                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
                                "not found".len(),
                                "not found"
                            );
                            // println!("{}", b);
                            stream.write_all(b.as_bytes());
                            stream.flush();
                        }
                    }
                    // println!("{}", html);
                }
                Static::Pages(pages) => {
                    for page in pages.iter() {
                        // println!("{}", page);
                        if path == *page {
                            let html = format!("{}{}/server.html", self.statics.0, page);
                            // println!("{}", html);
                            let b = page!(self.statics.0, html);
                            // println!("{}", &b.clone().unwrap());
                            let billionaires = match b {
                                Some(billionaire) => {
                                    let b = format!(
                                        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                                        billionaire.len(),
                                        billionaire
                                    );
                                    // println!("{}", b);
                                    b
                                }
                                None => {
                                    let b = format!(
                                        "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                                        "not found".len(),
                                        "not found"
                                    );
                                    b
                                }
                            };
                            stream.write_all(billionaires.as_bytes());
                            stream.flush();
                        } else if !pages.contains(&path) {
                            let b = format!(
                                "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                                "notfound".len(),
                                "notfound"
                            );
                            stream.write_all(b.as_bytes());
                            stream.flush();
                        }
                    }
                }
                Static::CustomePage(pages, notfound) => {
                    for page in pages.iter() {
                        if path == *page {
                            let page = page!(self.statics.0, page);
                            match page {
                                Some(page) => {
                                    let b = format!(
                                        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                                        page.len(),
                                        page
                                    );
                                    stream.write_all(b.as_bytes());
                                    stream.flush();
                                }
                                None => {
                                    let notfound = format!(
                                        "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                                        notfound.len(),
                                        notfound
                                    );
                                    stream.write_all(notfound.as_bytes());
                                    stream.flush();
                                }
                            }
                        } else if !pages.contains(&path) {
                            let b = format!(
                                "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                                notfound.len(),
                                notfound
                            );
                            stream.write_all(b.as_bytes());
                            stream.flush();
                        }
                    }
                }
                Static::AllAndCustome404(page) => {
                    let html = format!("app{}/server.html", path);
                    let b = page!(self.statics.0, html);
                    match b {
                        Some(b) => {
                            let b = format!(
                                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                                b.len(),
                                b
                            );
                            // println!("{}", b);
                            stream.write_all(b.as_bytes());
                            stream.flush();
                        }
                        None => {
                            let b = format!(
                                "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                                page.len(),
                                page
                            );
                            // println!("{}", b);
                            stream.write_all(b.as_bytes());
                            stream.flush();
                        }
                    }
                }
            }
            return;
        }
        // let path = path.to_string();
        // thread::spawn(move || {
        if read_response.contains("Sec-WebSocket") {
            let buffer_read = buffer[..b].to_vec();
            let prepend_buffer = PrependBuffer {
                prepend_buffer: Cursor::new(buffer_read),
                inner: stream,
            };
            let ws = accept(prepend_buffer);
            match ws {
                Ok(mut b) => {
                    for route in self.ws_routes.iter() {
                        route(&mut b, path.to_string());
                    }
                }
                Err(err) => {
                    println!("{}", err)
                }
            }
        }
        // });
        // thread::spawn(move || {

        // })
        // println!("{}", "billionaire");
        // for route in self.routes.iter() {
        //     // thread::spawn(move || {
        //     route(stream, &response, cp, &cors_header, custome_headers.clone());
        //     // });
        // }
        // }
        //     false => {}
        // }
    }
}

impl<R> Read for PrependBuffer<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.prepend_buffer.position() < self.prepend_buffer.get_ref().len() as u64 {
            return self.prepend_buffer.read(buf);
        }
        self.inner.read(buf)
    }
}

impl<R> Write for PrependBuffer<R>
where
    R: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
