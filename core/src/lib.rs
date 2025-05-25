pub use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
use std::{io::Cursor, sync::Arc};

// use ::threadpool::ThreadPool;
pub use brotli;
use brotli::CompressorWriter;
pub use flate2;
pub use http_scrap::{HMap, QMap, Response};
use minify_html::{Cfg, minify};
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
    pub fn message<'great>(&'great mut self, message: impl Into<String>) -> GateKeeperResponse {
        self.message = message.into();

        GateResponse::response(Return {
            status: self.status,
            content_type: self.content_type.clone(),
            headers: Some(self.header.clone()),
            message: self.message.clone(),
            encoding: self.encoding.clone(),
        })
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

#[derive(Clone)]
pub enum Middleware {
    Cors(String),
    Compression(String),
    CustomeHeader(String, String), // Jwt(bool),
}

impl Middleware {
    pub fn cors<B: Into<String>>(cors: B) -> Self {
        Self::Cors(cors.into())
    }
    pub fn compression<B: Into<String>>(compression: B) -> Self {
        Self::Compression(compression.into())
    }
    pub fn custome_header<B: Into<String>>(key: B, value: B) -> Self {
        Middleware::CustomeHeader(key.into(), value.into())
    }
}

#[derive(Clone)]
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
    Redirect(u64, String),
}

pub struct GateKeeper;
impl GateKeeper {
    pub fn next() -> GateKeeperResponse {
        GateKeeperResponse::Next
    }
    pub fn response(response: Return) -> GateKeeperResponse {
        GateKeeperResponse::Response(response)
    }
    pub fn redirect(status: u64, redirect: impl Into<String>) -> GateKeeperResponse {
        GateKeeperResponse::Redirect(status, redirect.into())
    }
}

struct GateResponse;
impl GateResponse {
    fn response(response: Return) -> GateKeeperResponse {
        GateKeeperResponse::Response(response)
    }
    fn redirect(status: u64, redirect: String) -> GateKeeperResponse {
        GateKeeperResponse::Redirect(status, redirect)
    }
}

pub struct GatesRedirect;
impl GatesRedirect {
    pub fn redirect(status: u64, redirect: impl Into<String>) -> GateKeeperResponse {
        GateResponse::redirect(status, redirect.into())
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

type Buck = fn(&TcpStream, &Response, &str, &str, Vec<(&str, &str)>);
type WsBuck = fn(&mut GatesDope, String);

#[derive(Clone)]
pub enum Static<'b> {
    All,
    Pages(&'b [&'b str]),
    CustomePage(&'b [&'b str], &'b str),
    AllAndCustome404(&'b str),
}

#[derive(Clone)]
pub struct Gates<'billionaire> {
    port: String,
    middleware: Vec<Middleware>,
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

impl<'gates> Gates<'static> {
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
    pub fn statics(mut self, dir: impl Into<String>, file: Static<'static>) -> Self {
        self.statics = (dir.into(), file);
        self
    }
    pub fn rate_limit(mut self, kind: RL<'static>) -> Self {
        self.rl.push(Some(kind));
        self
    }
    // pub fn ws_routes(mut self, ws: &[WsBuck]) -> Self {
    //     self.ws_routes = ws.to_vec();
    //     self
    // }
    pub fn middleware(mut self, middleware: Middleware) -> Self {
        self.middleware.push(middleware);
        self
    }
    pub fn run(self) {
        let listener = TcpListener::bind(&self.port).unwrap();
        // listener.set_nonblocking(true);
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
        while let Ok((stream, _)) = listener.accept() {
            let stream = stream;
            // let value = self.handle_connection(&stream, &mut s);
            // let b = b_clone.lock().unwrap();
            let b = Arc::new(self.clone());
            let mut s = s.clone();
            gates_threads.execute(move || {
                b.handle_connection(&stream, &mut s);
                // b_clone;
                // value;
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
        let b = stream.read(&mut buffer);

        match b {
            Ok(b) => {
                if b == 0 {
                    return;
                }
                let buffer = &buffer[..b];

                let read_response = String::from_utf8_lossy(&buffer);

                let response = Response::new(&read_response.to_string());
                let path = response.path();

                // region: --- RateLimit
                Self::ratelimit(stream, ratelimit, path);
                // endregion: --- RateLimit

                let mut cp = "";
                let mut cors_header = String::new();

                let mut custome_headers = Vec::new();

                // region: --- Middleware
                Self::middleware_function(
                    &self.middleware,
                    read_response.to_string(),
                    stream,
                    &mut cp,
                    &mut custome_headers,
                    &mut cors_header,
                );
                // endregion: --- Middleware

                // region: --- page
                if !path.starts_with("/api") && !path.starts_with("/ws") {
                    Self::page(&self, path, stream);
                }
                // endregion: --- page

                // region: --- WebSocket
                if read_response.contains("Sec-WebSocket") {
                    Self::websocket(&self, buffer, b, stream, path);
                }
                // endregion: --- WebSocket

                // region: --- rest
                for route in self.routes.iter() {
                    route(stream, &response, cp, &cors_header, custome_headers.clone());
                }
                // endregion: --- rest
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => return,
            Err(err) => {
                println!("{}", err);
            }
        }
        // }
        //     false => {}
        // }
    }
    fn ratelimit(
        mut stream: &TcpStream,
        ratelimit: &mut Vec<(Option<FixedLimit>, &str)>,
        path: &str,
    ) {
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
                            let _ = stream.write_all(page.as_bytes());
                            let _ = stream.flush();
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
                                let _ = stream.write_all(page.as_bytes());
                                let _ = stream.flush();
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
                            let _ = stream.write_all(page.as_bytes());
                            let _ = stream.flush();
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
    }
    fn middleware_function(
        middleware: &'gates Vec<Middleware>,
        read_response: String,
        mut stream: &TcpStream,
        cp: &mut &'gates str,
        custome_headers: &mut Vec<(&'gates str, &'gates str)>,
        cors_header: &mut String,
    ) {
        for middleware in middleware.iter() {
            match middleware {
                Middleware::Cors(cors) => {
                    let b = format!(
                        "HTTP/1.1 204 No Content\r\n{}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                        cors
                    );
                    // println!("{}", b);
                    cors_header.push_str(cors);
                    if read_response.starts_with("OPTIONS") {
                        // println!("{}", b);
                        let _ = stream.write_all(b.as_bytes());
                        let _ = stream.flush();
                    }
                }
                Middleware::Compression(types) => {
                    let b = types.as_str();
                    *cp = b;
                }
                Middleware::CustomeHeader(header, value) => {
                    custome_headers.push((header.as_str(), value.as_str()));
                }
            }
        }
    }
    fn page(&self, path: &str, stream: &TcpStream) {
        let bmode = &self.statics.1;
        let cfg = Cfg::new();
        match bmode {
            Static::All => {
                let html = format!("{}{}/server.html", self.statics.0, path);
                // println!("{}", html);
                let b = page!(self.statics.0, html);
                // println!("{}", b.as_ref().unwrap());
                match b {
                    Some(b) => {
                        Self::compression(stream, &cfg, b);
                    }
                    None => {
                        Self::not_found(stream, "not found");
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
                        match b {
                            Some(billionaire) => {
                                Self::compression(stream, &cfg, billionaire);
                            }
                            None => {
                                Self::not_found(stream, page);
                            }
                        };
                    } else if !pages.contains(&path) {
                        Self::not_found(stream, "notfound");
                    }
                }
            }
            Static::CustomePage(pages, notfound) => {
                for page in pages.iter() {
                    if path == *page {
                        let page = page!(self.statics.0, page);
                        match page {
                            Some(page) => {
                                Self::compression(stream, &cfg, page);
                            }
                            None => {
                                Self::not_found(stream, &notfound);
                            }
                        }
                    } else if !pages.contains(&path) {
                        Self::not_found(stream, page);
                    }
                }
            }
            Static::AllAndCustome404(page) => {
                let html = format!("app{}/server.html", path);
                let b = page!(self.statics.0, html);
                match b {
                    Some(b) => {
                        Self::compression(stream, &cfg, b);
                    }
                    None => {
                        Self::not_found(stream, page);
                    }
                }
            }
        }
        return;
    }
    fn websocket(&self, buffer: &[u8], b: usize, stream: &TcpStream, path: &str) {
        let buffer_read = buffer[..b].to_vec();
        let prepend_buffer = PrependBuffer {
            prepend_buffer: Cursor::new(buffer_read),
            inner: stream,
        };
        // let b = String::from_utf8_lossy(pre);
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
    fn compression(mut stream: &TcpStream, cfg: &Cfg, b: String) {
        let compressed = minify(b.as_bytes(), &cfg);
        let mut best = Vec::new();
        {
            let mut brotli_compressed = CompressorWriter::new(&mut best, 4096, 5, 22);
            let _ = brotli_compressed.write_all(&compressed);
        }
        let b = format!(
            "HTTP/1.1 200 OK\r\nContent-Encoding: br\r\nConnection: keep-alive`\r\nContent-Type: text/html\r\n\r\n",
        );
        // println!("{}", b);
        let _ = stream.write_all(b.as_bytes());

        let _ = stream.write_all(&mut best);
        let _ = stream.flush();
    }
    fn not_found(mut stream: &TcpStream, page: &str) {
        let b = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            page.len(),
            page
        );
        // println!("{}", b);
        let _ = stream.write_all(b.as_bytes());
        let _ = stream.flush();
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
