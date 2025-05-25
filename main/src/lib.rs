
use proc_macro::{ TokenStream};
use quote::quote;
use syn::{self, parse_macro_input, parse_str, Ident, ItemFn};

#[proc_macro_attribute]
pub fn gates(attr: TokenStream, func: TokenStream) -> TokenStream {
    // println!("================================{}", attr);
    let params = attr.to_string();
    let (method, path,function) = parse_attribute(params);

    // println!("mmmmmmmmmmmmmmmmmm{:?}",function);
    // let functions:Ident;
    // if let Some(function) = function {
    //     functions = function;
    // }
    let block = parse_macro_input!(func as ItemFn);
    let fn_name = &block.sig.ident;
    // let fnt = &block.sig.fn_token;
    let is_keeper = &function.is_some();

    // println!("{}",method);
    // println!("============================== {:?}",function);

    // let b = &function.unwrap();
    // let fnb = &b;
    // let name = &b.sig.ident;

    // println!("{:?}", b);
    // let block = block.block;
    // println!("{}", fn_name);
    // println!("{:?}",b.span());
    // println!("billionaire {}",method);

    // let zen = Ident::new("billion",Span::call_site().into());

    // let b = quote!{

    // };
    // println!("{:?}", block);
    // println!("{}", method);
    // println!("{}", path);
    // println!("{}", path.unwrap());
    // println!("{}", kind.unwrap());
    // let mut method = "";
    // println!("{}", kind);
    // println!("{}", method);
    // let method = b.nth(0);
    // let path = b.nth(0);
    // println!("{}", method);
    // println!("{}", path.unwrap());
    // match param[0] {
    //     "GET" => method = "GET",
    //     "POST" => method = "POST",
    //     "DELETE" => method = "DELETE",
    //     "PUT" => method = "PUT",
    //     _ => method = "GET",
    // }
    // println!("{}", method);

    // let path = "";
    // let attrs = attr.
    // println!("{}", func);

    match is_keeper {
      true => {
        // println!("========================================== {}",method);

        match method.as_str() {
          "GET" => {
              quote! {
                  // use nous::HashMap;
                  // use nous::Response;
                  // use nous::TcpStream;
                  // use nous::GatesRequest;
                  // use std::io::BufWriter;
                  // use nous::flate2::write::GzEncoder;
                  // use nous::flate2::Compression;
                  // use nous::brotli::CompressorWriter;
                  // use std::io::Cursor;
                  // use nous::zstd::encode_all;
                  // use nous::Write;
                  // use nous::GateKeeperResponse;
                  // use nous::TcpStream;
                  // use nous::Response;
                  // use nous::HashMap;
                  // use nous::Write;
                  // use nous::HashMap;
                  // use nous::Response;
                  // use nous::TcpStream;
                  // use nous::GatesRequest;
                  // use std::io::BufWriter;
                  // use nous::flate2::write::GzEncoder;
                  // use nous::flate2::Compression;
                  // use nous::brotli::CompressorWriter;
                  // use std::io::Cursor;
                  // use nous::zstd::encode_all;
                  // use nous::Write;
                  // use nous::GateKeeperResponse;
                  // use nous::GateKeeperResponse;
                  use nous::TcpStream;
                  use nous::Response;
                  use nous::HashMap;
                  use nous::Write;
                  pub fn #fn_name(mut stream:&TcpStream,response:&Response,compression:&str,secure_header:&str,header:Vec<(&str, &str)>) {
                      mod bucket {
                          use nous::HashMap;
                          use nous::Response;
                          use nous::TcpStream;
                          use nous::GatesRequest;
                          use std::io::BufWriter;
                          use nous::flate2::write::GzEncoder;
                          use nous::flate2::Compression;
                          use nous::brotli::CompressorWriter;
                          use std::io::Cursor;
                          use nous::zstd::encode_all;
                          use nous::Write;
                          // use nous::HashMap;
                          // use nous::GateKeeperResponse;
                          use nous::Return;
  
                          pub fn gzip_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = GzEncoder::new(Vec::new(), Compression::default());
                              b.write_all(&billionaire.message.as_bytes()).unwrap();
                              let b = b.finish();
                              if let Ok(response) = b {
                                  for b in response.chunks(6) {
                                      // hexa of chunk\r\n
                                      // chunk\r\n
                                      let billionaire = format!("{:X}\r\n", b.len());
                                      // println!("{}",billionaire);
                                      let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                      let _ = stream.write_all(b).unwrap();
                                      // println!("{:?}",b);
                                      let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                  }
                              }
                          }
                          pub fn brotli_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = Vec::new();
                              {
                                  let mut b = CompressorWriter::new(&mut b, 4096, 4, 22);
                                  b.write_all(billionaire.message.as_bytes()).unwrap();
                                  b.flush().unwrap();
                              }
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn zstd_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut input = Cursor::new(&billionaire.message);
                              let b = encode_all(&mut input, 3).unwrap();
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn param_parser(parts:Vec<&str>,user_parts:Vec<&str>, bhash:&mut HashMap<String,String>){
                              for (b, billion) in parts.iter().zip(user_parts.iter()) {
                                  // println!("{}{}", b, billion);
                                  if b != billion && billion.starts_with(&"<") && billion.ends_with(&">") {
                                      let key = billion.replace("<", "").replace(">", "");
                                      let value = b;
                                      bhash.insert(key.to_string(), value.to_string());
                                  }
                              }
                          }
                      pub fn zen(response:Return,mut stream:&TcpStream,header:Vec<(&str, &str)>,compression:&str,secure_header:&str) {
                              // println!("{}",pathb);
                      // println!("{}",compression);
                      // println!("{:?}",response.encoding);
                      // println!("{}","billionairegreathari");
                      let mut chunks = BufWriter::new(&mut stream);
                      
                      let billionaire = response;
                      // Return { status: 200, message: ["{\"name\":\"billionaireid\"}"], content_type: Some("application/json"), headers: Some([]) }
                      
                      let mut response = format!("HTTP/1.1 {} OK\r\n\
                                  Connection: keep-alive\r\n\
                                  X-Content-Type-Options: nosniff\r\n\
                                  Transfer-Encoding: chunked\r\n",&billionaire.status);
                      if secure_header.len() != 0 {
                        response.push_str(&format!("{}\r\n",secure_header));
                      }
                      
                      // println!("{:?}",header); 
                      if header.len() >= 1 {
                        for header in header.iter(){
                          // println!("{:?}",header);;
                          response.push_str(&format!("{}: {}\r\n",header.0,header.1));
                        }
                      }                               
                      if !response.contains("Content-Security-Policy: "){
                        response.push_str(&format!("Content-Security-Policy: default-src 'self'\r\n"));
                      }
                      if !response.contains("X-Frame-Options: "){
                        response.push_str(&format!("X-Frame-Options: DENY\r\n"));
                      }
                      if !response.contains("Referrer-Policy: "){
                        response.push_str(&format!("Referrer-Policy: no-referrer\r\n"));
                      }
                      if !response.contains("Keep-Alive: "){
                        response.push_str(&format!("Keep-Alive: timeout=10, max=100\r\n"));
                      }
                      if !response.contains("Cache-Control: "){
                        response.push_str(&format!("Cache-Control: public; max-age: 5; stale-while-revalidate: 10\r\n"));
                      }
                      if let Some(res) = &billionaire.content_type {
                      //   println!("{}",res);
                        response.push_str(&format!("Content-Type: {}; charset=UTF-8\r\n",res));
                      }
                      
                      
                      // println!("{}",response);
                      
                    if compression == "" && billionaire.encoding.is_none(){
                      // println!("{}","billionaire");
                      response.push_str("Content-Encoding: gzip");
                      response.push_str("\r\n\r\n");
                      // println!("{}",response);
                      chunks.write_all(response.as_bytes()).unwrap();
                      // println!("{:?}",billionaire);
                      gzip_compress(&billionaire,&mut chunks);
                    }
                    if let Some(b) = &billionaire.encoding {
                        if b == "gzip" {
                          response.push_str("Content-Encoding: gzip");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          // println!("{:?}",billionaire);
                          gzip_compress(&billionaire,&mut chunks);
  
                        }
                        else if b == "brotli" {
                          response.push_str("Content-Encoding: br");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          brotli_compress(&billionaire,&mut chunks);
                        }
                        else if compression == "zstd" {
                          response.push_str("Content-Encoding: zstd");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          zstd_compress(&billionaire,&mut chunks);
  
  
                        }
                    }
                    else {
                      if compression == "gzip" {
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      else if compression == "brotli" {
                        response.push_str("Content-Encoding: br");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        brotli_compress(&billionaire,&mut chunks);
                      }
                      else if compression == "zstd" {
                        response.push_str("Content-Encoding: zstd");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        zstd_compress(&billionaire,&mut chunks);
                      }
                    }
                      chunks.write_all("0\r\n\r\n".as_bytes()).unwrap();
                      chunks.flush().unwrap();     
                      
                    }
              }
  
                      #block
                      // if let Some(function) = 
                      // println!("{:?}",b);
                      
                      // match b {
                      //     Ok(b) => {
                              // println!("{}",b);
                      //     }
                      //     Err(error) => {
                              
                      //         let b = format!("HTTP/1.1 {} NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",error.status,error.message.len(),error.message);
                              // println!("{}",b);
                      //         let _ = stream.write_all(b.as_bytes());
                      //         let _ = stream.flush();
                      //         // return;
                      //     }
                      // }
                      //     let gatekeeper = {#function()};
                      //     println!("{:?}",function);
                      // };
                      let methodb = response.method();
                      let pathb = response.path();
                      let len = pathb.find("?").or_else(|| Some(pathb.len())).unwrap();
                      let real_path = &pathb[..len];
                      // println!("from {}",pathb);
                      // println!("{}",compression);
                      // println!("billionaire {}",#path);
                      // println!("{}",pathb == #path);
          
                      // println!("{}",real_path);
  
                      let parts:Vec<&str> = pathb.split("/").collect();
                      // println!("{:?}",parts);
                      let user_parts:Vec<&str> = #path.split("/").collect();
                      // println!("bbbbbb {}",parts.len() == user_parts.len());
                      // println!("{:?}",user_parts);
          
                      let mut billionaires = HashMap::new();
          
                      if methodb == #method {
                          // println!("{}",methodb);
                          // println!("{}","++++++++++++++++++++++++++");
                          if real_path == #path {
                              // println!("========={}","billionairegreat");
                              // println!("{:?}",response.query());
                              let billionaire = GatesRequest {
                                  body: "",
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query(),
                              };
                              // println!("{:?}",billionaire);
                              // let response = {#fn_name(&billionaire)};
                              if #is_keeper {
                                let gatekeeper = {#function(&billionaire)};
                                // println!("{:?}",gatekeeper);
                                // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                                // status
                                // redirect
                                if let GateKeeperResponse::Response(b) = gatekeeper {
                                    let mut html = format!("HTTP/1.1 {} NotFound\r\n",b.status);
                                    // Return { status: 404, message: "", content_type: Some("application/json"), headers: Some([]), encoding: None }
                                    if let Some(headers) = b.headers {
                                        for header in header.iter() {
                                            html.push_str(&format!("{}: {}\r\n",header.0,header.1));
                                        }
                                    }
                                    if let Some(content_type) = b.content_type {
                                        html.push_str(&format!("Content-Type: {}\r\n",content_type));
                                    }
                                    if let Some(encoding) = b.encoding {
                                        html.push_str(&format!("Content-Encoding: {}\r\n",encoding));
                                    }
                                    html.push_str(&format!("Content-Length: {}\r\n\r\n{}",b.message.len(),b.message));
                                    // println!("{}",html);
                                    let _ = stream.write_all(html.as_bytes()).unwrap();
                                    let _ = stream.flush().unwrap();
                                    // println!("{}","finish");
                                    return;
                                }
                                if let GateKeeperResponse::Redirect(b,path) = gatekeeper {
                                  let b = format!("HTTP/1.1 {} Temporary Redirect\r\nLocation: {}\r\nConnection: close\r\n\r\n",b,path);
                                  // println!("{}",b);
                                  let _ = stream.write_all(b.as_bytes()).unwrap();
                                  let _ = stream.flush().unwrap();
                                  return;
                                  // println!("{}","billionairegreat");
                                }
                                if let GateKeeperResponse::Next = gatekeeper {
                                  let response = {#fn_name(&billionaire)};
                                  if let GateKeeperResponse::Response(response) = response {
                                    bucket::zen(response,stream,header,compression,secure_header);
                                  }   
                                  else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                    let status = match code {
                                      302 => "Temproary Redirect",
                                      301 => "Permanently Moved",
                                      307 => "Temproary Moved",
                                      _ => "OK"
                                    };
                                    let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                    let _ = stream.write_all(response.as_bytes()).unwrap();
                                    let _ = stream.flush().unwrap();
                                    // println!("{}",response);
                                    return;
                                  }             
                                }

                              }
                              // println!("{}","peek");
                                  // chunks.write_all(billionaire.0.as_bytes());
                                  // // chunks.write_all(&compress);
                                  // // println!("{:?}",b);
                          }
                        // }
                        else if #path.contains("<") && #path.contains(">")  && !["/favicon.ico"].contains(&pathb) && parts.len() == user_parts.len() {
                              // #b;
                              // println!("{}","billionaire++++++++++++++++++++++++++");
                              // println!("{}","billionaires");
                              bucket::param_parser(parts,user_parts,&mut billionaires);
                              // println!("======={:?}",billionaires);
                              let billionaire = GatesRequest {
                                  body: "",
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query()
                              };
  
                              if #is_keeper {
                                  let gatekeeper = {#function(&billionaire)};
                                  // println!("{}","billionairegreat");
                                  //   // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                                  if let GateKeeperResponse::Response(b) = gatekeeper {
                                     let mut html = format!("HTTP/1.1 {} {}\r\n",b.status,b.status);
                                        // Return { status: 404, message: "", content_type: Some("application/json"), headers: Some([]), encoding: None }
                                      if secure_header.len() != 0 {
                                        html.push_str(&format!("{}\r\n",secure_header));
                                      }
                                      if let Some(headers) = b.headers {
                                          for header in header.iter() {
                                            html.push_str(&format!("{}: {}\r\n",header.0,header.1));
                                          }
                                      }
                                      if let Some(content_type) = b.content_type {
                                          html.push_str(&format!("Content-Type: {}\r\n",content_type));
                                      }
                                      if let Some(encoding) = b.encoding {
                                          html.push_str(&format!("Content-Encoding: {}\r\n",encoding));
                                      }
                                      html.push_str(&format!("Content-Length: {}\r\n\r\n{}",b.message.len(),b.message));
                                            // println!("{}",html);
                                      let _ = stream.write_all(html.as_bytes()).unwrap();
                                      let _ = stream.flush().unwrap();
                                            // println!("{}","finish");
                                      return; 
                                  }
                                  if let GateKeeperResponse::Redirect(b,path) = gatekeeper {
                                    let mut b = format!("HTTP/1.1 {} Permanent Redirect\r\n",b);
                                    if secure_header.len() != 0 {
                                      b.push_str(&format!("{}\r\n",secure_header));
                                    }
                                    b.push_str(&format!("Location: {}\r\n\r\n",path));
                                    let _ = stream.write_all(b.as_bytes()).unwrap();
                                    let _ = stream.flush().unwrap();
                                    // println!("{}",b);
                                    return;
                                  }
                                  if let GateKeeperResponse::Next = gatekeeper {
                                    let response = {#fn_name(&billionaire)};
                                    if let GateKeeperResponse::Response(response) = response {
                                      bucket::zen(response,stream,header,compression,secure_header);
                                    }   
                                    else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                      let status = match code {
                                        302 => "Temproary Redirect",
                                        301 => "Permanently Moved",
                                        307 => "Temproary Moved",
                                        _ => "OK"
                                      };
                                      let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                      let _ = stream.write_all(response.as_bytes()).unwrap();
                                      let _ = stream.flush().unwrap();
                                      // println!("{}",response);
                                      return;
                                    }             
                                  }
                                }
                        }
                      }
                    }
                  }
              .into()
          }
          "POST" => {
              quote! {
                  // use nous::HashMap;
                  // use nous::Response;
                  // // use nous::TcpStream;
                  // use nous::GatesRequest;
                  // use std::io::BufWriter;
                  // use nous::flate2::write::GzEncoder;
                  // use nous::flate2::Compression;
                  // use nous::brotli::CompressorWriter;
                  // use std::io::Cursor;
                  // use nous::zstd::encode_all;
                  // use nous::Write;
                  // use nous::GateKeeperResponse;
                  pub fn #fn_name(mut stream:&TcpStream,response:&Response,compression:&str,secure_header:&str,header:Vec<(&str, &str)>) {
                      mod bucket {
                          use nous::HashMap;
                          use nous::Response;
                          use nous::TcpStream;
                          use nous::GatesRequest;
                          use std::io::BufWriter;
                          use nous::flate2::write::GzEncoder;
                          use nous::flate2::Compression;
                          use nous::brotli::CompressorWriter;
                          use std::io::Cursor;
                          use nous::zstd::encode_all;
                          use nous::Write;
                          // use nous::GateKeeperResponse;
                          use nous::Return;
                          pub fn gzip_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = GzEncoder::new(Vec::new(), Compression::default());
                              b.write_all(&billionaire.message.as_bytes()).unwrap();
                              let b = b.finish();
                              if let Ok(response) = b {
                                  for b in response.chunks(6) {
                                      // hexa of chunk\r\n
                                      // chunk\r\n
                                      let billionaire = format!("{:X}\r\n", b.len());
                                      // println!("{}",billionaire);
                                      let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                      let _ = stream.write_all(b).unwrap();
                                      // println!("{:?}",b);
                                      let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                  }
                              }
                          }
                          pub fn brotli_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = Vec::new();
                              {
                                  let mut b = CompressorWriter::new(&mut b, 4096, 11, 22);
                                  b.write_all(billionaire.message.as_bytes()).unwrap();
                                  b.flush().unwrap();
                              }
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn zstd_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut input = Cursor::new(&billionaire.message);
                              let b = encode_all(&mut input, 3).unwrap();
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn param_parser(parts:Vec<&str>,user_parts:Vec<&str>, bhash:&mut HashMap<String,String>){
                              for (b, billion) in parts.iter().zip(user_parts.iter()) {
                                  // println!("{}{}", b, billion);
                                  if b != billion && billion.starts_with(&"<") && billion.ends_with(&">") {
                                      let key = billion.replace("<", "").replace(">", "");
                                      let value = b;
                                      bhash.insert(key.to_string(), value.to_string());
                                  }
                              }
                          }
                          pub fn zen(response:Return,mut stream:&TcpStream,header:Vec<(&str, &str)>,compression:&str,secure_header:&str) {
                              // println!("{}",pathb);
                      // println!("{}",compression);
                      // println!("{}","billionairegreathari");
                      let mut chunks = BufWriter::new(&mut stream);
                      
                      let billionaire = response;
                      // Return { status: 200, message: ["{\"name\":\"billionaireid\"}"], content_type: Some("application/json"), headers: Some([]) }
                      
                      let mut response = format!("HTTP/1.1 {} OK\r\n\
                                  Connection: keep-alive\r\n\
                                  X-Content-Type-Options: nosniff\r\n\
                                  Transfer-Encoding: chunked\r\n",&billionaire.status);
                      
                      // println!("{}",secure_header);
                      if secure_header.len() != 0 {
                        response.push_str(&format!("{}\r\n",secure_header));
                      }
                      // println!("{:?}",header); 
                      if header.len() >= 1 {
                        for header in header.iter(){
                          // println!("{:?}",header);;
                          response.push_str(&format!("{}: {}\r\n",header.0,header.1));
                        }
                      }                               
                      if !response.contains("Content-Security-Policy: "){
                        response.push_str(&format!("Content-Security-Policy: default-src 'self'\r\n"));
                      }
                      if !response.contains("X-Frame-Options: "){
                        response.push_str(&format!("X-Frame-Options: DENY\r\n"));
                      }
                      if !response.contains("Referrer-Policy: "){
                        response.push_str(&format!("Referrer-Policy: no-referrer\r\n"));
                      }
                      if !response.contains("Keep-Alive: "){
                        response.push_str(&format!("Keep-Alive: timeout=10, max=100\r\n"));
                      }
                      if !response.contains("Cache-Control: "){
                        response.push_str(&format!("Cache-Control: public; max-age: 5; stale-while-revalidate: 10\r\n"));
                      }
                      if let Some(res) = &billionaire.content_type {
                      //   println!("{}",res);
                        response.push_str(&format!("Content-Type: {}; charset=UTF-8\r\n",res));
                      }
                      
                      
                      // println!("{}",response);
                      if compression == "" && billionaire.encoding.is_none(){
                        // println!("{}","billionaire");
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        // println!("{:?}",billionaire);
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      if let Some(b) = &billionaire.encoding {
                        if b == "gzip" {
                          response.push_str("Content-Encoding: gzip");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          // println!("{:?}",billionaire);
                          gzip_compress(&billionaire,&mut chunks);
  
                        }
                        else if b == "brotli" {
                          response.push_str("Content-Encoding: br");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          brotli_compress(&billionaire,&mut chunks);
                        }
                        else if compression == "zstd" {
                          response.push_str("Content-Encoding: zstd");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          zstd_compress(&billionaire,&mut chunks);
  
  
                        }
                      }
                      else {
                      if compression == "gzip" {
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      else if compression == "brotli" {
                        response.push_str("Content-Encoding: br");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        brotli_compress(&billionaire,&mut chunks);
  
  
                      }
                      else if compression == "zstd" {
                        response.push_str("Content-Encoding: zstd");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        zstd_compress(&billionaire,&mut chunks);
                      }
                      }
                      chunks.write_all("0\r\n\r\n".as_bytes()).unwrap();
                      chunks.flush().unwrap();     
                      
                      }
                      }
  
                      #block

                      let methodb = response.method();
                      let pathb = response.path();
                      let len = pathb.find("?").or_else(|| Some(pathb.len())).unwrap();
                      let real_path = &pathb[..len];
                      // println!("from {}",pathb);
                      // println!("{}",compression);
                      // println!("billionaire {}",#path);
                      // println!("{}",pathb == #path);
          
                      let parts:Vec<&str> = pathb.split("/").collect();
                      // println!("{:?}",parts);
                      let user_parts:Vec<&str> = #path.split("/").collect();
                      // println!("bbbbbb {}",parts.len() == user_parts.len());
                      // println!("{:?}",user_parts);
          
                      let mut billionaires = HashMap::new();
          
                      if methodb == #method {
                          // println!("{}",methodb);
                          if real_path == #path {
                              // println!("{}","billionairegreat");
                              // println!("{:?}",response);
                              let billionaire = GatesRequest {
                                  body: response.content(),
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query(),
                              };
                              let response = {#fn_name(&billionaire)};
                              if #is_keeper {
                                let gatekeeper = {#function(&billionaire)};
                                // println!("{}","billionairegreat");
                                // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                                if let GateKeeperResponse::Response(b) = gatekeeper {
                                    let mut html = format!("HTTP/1.1 {} {}\r\n",b.status,b.status);
                                    // Return { status: 404, message: "", content_type: Some("application/json"), headers: Some([]), encoding: None }
                                    if secure_header.len() != 0 {
                                      html.push_str(&format!("{}\r\n",secure_header));
                                    }
                                    if let Some(headers) = b.headers {
                                        for header in header.iter() {
                                            html.push_str(&format!("{}: {}\r\n",header.0,header.1));
                                        }
                                    }
                                    if let Some(content_type) = b.content_type {
                                        html.push_str(&format!("Content-Type: {}\r\n",content_type));
                                    }
                                    if let Some(encoding) = b.encoding {
                                        html.push_str(&format!("Content-Encoding: {}\r\n",encoding));
                                    }
                                    html.push_str(&format!("Content-Length: {}\r\n\r\n{}",b.message.len(),b.message));
                                    // println!("{}",html);
                                    let _ = stream.write_all(html.as_bytes()).unwrap();
                                    let _ = stream.flush().unwrap();
                                    // println!("{}","finish");
                                    return; 
                                }
                                if let GateKeeperResponse::Redirect(b,path) = gatekeeper {
                                  // let secure_header = format!("{}\r\n",secure_header);
                                  let mut b = format!("HTTP/1.1 {} Permanent Redirect\r\n",b);
                                  if secure_header.len() != 0 {
                                    b.push_str(&format!("{}\r\n",secure_header));
                                  }
                                  b.push_str(&format!("Location: {}\r\n\r\n",path));
                                  let _ = stream.write_all(b.as_bytes()).unwrap();
                                  let _ = stream.flush().unwrap();
                                  return;
                                }
                                  if let GateKeeperResponse::Next = gatekeeper {
                                    let response = {#fn_name(&billionaire)};
                                    if let GateKeeperResponse::Response(response) = response {
                                      bucket::zen(response,stream,header,compression,secure_header);
                                    }   
                                    else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                      let status = match code {
                                        302 => "Temproary Redirect",
                                        301 => "Permanently Moved",
                                        307 => "Temproary Moved",
                                        _ => "OK"
                                      };
                                      let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                      let _ = stream.write_all(response.as_bytes()).unwrap();
                                      let _ = stream.flush().unwrap();
                                      // println!("{}",response);
                                      return;
                                    }             
                                  }
                              }
                                  // chunks.write_all(billionaire.0.as_bytes());
                                  // // chunks.write_all(&compress);
                                  // // println!("{:?}",b);
                          }
                          else if #path.contains("<") && #path.contains(">")  && !["/favicon.ico"].contains(&pathb) && parts.len() == user_parts.len() {
                              // #b;
                              // println!("{}","billionaires");
                              bucket::param_parser(parts,user_parts,&mut billionaires);
                              // println!("{:?}",billionaires);
                              let billionaire = GatesRequest {
                                  body: response.content(),
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query()
                                  };
                                  if #is_keeper {
                                    let gatekeeper = {#function(&billionaire)};
                                    // println!("{}","billionairegreat");
                                    // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                                    if let GateKeeperResponse::Response(b) = gatekeeper {
                                        let mut html = format!("HTTP/1.1 {} {}\r\n",b.status,b.status);
                                        // Return { status: 404, message: "", content_type: Some("application/json"), headers: Some([]), encoding: None }
                                                                            if secure_header.len() != 0 {
                                      html.push_str(&format!("{}\r\n",secure_header));
                                    }
                                        if let Some(headers) = b.headers {
                                            for header in header.iter() {
                                                html.push_str(&format!("{}: {}\r\n",header.0,header.1));
                                            }
                                        }
                                        if let Some(content_type) = b.content_type {
                                            html.push_str(&format!("Content-Type: {}\r\n",content_type));
                                        }
                                        if let Some(encoding) = b.encoding {
                                            html.push_str(&format!("Content-Encoding: {}\r\n",encoding));
                                        }
                                        html.push_str(&format!("Content-Length: {}\r\n\r\n{}",b.message.len(),b.message));
                                        // println!("{}",html);
                                        let _ = stream.write_all(html.as_bytes()).unwrap();
                                        let _ = stream.flush().unwrap();
                                        // println!("{}","finish");
                                        return; 
                                    }
                                    if let GateKeeperResponse::Redirect(b,path) = gatekeeper {
                                  let mut b = format!("HTTP/1.1 {} Permanent Redirect\r\n",b);
                                  if secure_header.len() != 0 {
                                    b.push_str(&format!("{}\r\n",secure_header));
                                  }
                                  b.push_str(&format!("Location: {}\r\n\r\n",path));
                                      let _ = stream.write_all(b.as_bytes()).unwrap();
                                      let _ = stream.flush().unwrap();
                                      return;
                                    }
                                    if let GateKeeperResponse::Next = gatekeeper {
                                      let response = {#fn_name(&billionaire)};
                                      if let GateKeeperResponse::Response(response) = response {
                                        bucket::zen(response,stream,header,compression,secure_header);
                                      }   
                                      else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                        let status = match code {
                                          302 => "Temproary Redirect",
                                          301 => "Permanently Moved",
                                          307 => "Temproary Moved",
                                          _ => "OK"
                                        };
                                        let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                        let _ = stream.write_all(response.as_bytes()).unwrap();
                                        let _ = stream.flush().unwrap();
                                        // println!("{}",response);
                                        return;
                                      }             
                                    }
                                  }
                          }
                      }
                  }
              }
              .into()
          }
          "PUT" => {
              quote! {
                  // use nous::HashMap;
                  // use nous::Response;
                  // // use nous::TcpStream;
                  // use nous::GatesRequest;
                  // use std::io::BufWriter;
                  // use nous::flate2::write::GzEncoder;
                  // use nous::flate2::Compression;
                  // use nous::brotli::CompressorWriter;
                  // use std::io::Cursor;
                  // use nous::zstd::encode_all;
                  // use nous::Write;
                  // use nous::GateKeeperResponse;
                  // use nous::TcpStream;
                  // use nous::Response;
                  // use nous::HashMap;
                  // use nous::Write;
                  pub fn #fn_name(mut stream:&TcpStream,response:&Response,compression:&str,secure_header:&str,header:Vec<(&str, &str)>) {
                      mod bucket {
                          use nous::HashMap;
                          use nous::Response;
                          use nous::TcpStream;
                          use nous::GatesRequest;
                          use std::io::BufWriter;
                          use nous::flate2::write::GzEncoder;
                          use nous::flate2::Compression;
                          use nous::brotli::CompressorWriter;
                          use std::io::Cursor;
                          use nous::zstd::encode_all;
                          use nous::Write;
                          // use nous::GateKeeperResponse;
                          // use crate::#function;
                          // use crate::#fn_name;
                          use nous::Return;
                          pub fn gzip_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = GzEncoder::new(Vec::new(), Compression::default());
                              b.write_all(&billionaire.message.as_bytes()).unwrap();
                              let b = b.finish();
                              if let Ok(response) = b {
                                  for b in response.chunks(6) {
                                      // hexa of chunk\r\n
                                      // chunk\r\n
                                      let billionaire = format!("{:X}\r\n", b.len());
                                      // println!("{}",billionaire);
                                      let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                      let _ = stream.write_all(b).unwrap();
                                      // println!("{:?}",b);
                                      let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                  }
                              }
                          }
                          pub fn brotli_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = Vec::new();
                              {
                                  let mut b = CompressorWriter::new(&mut b, 4096, 11, 22);
                                  b.write_all(billionaire.message.as_bytes()).unwrap();
                                  b.flush().unwrap();
                              }
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn zstd_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut input = Cursor::new(&billionaire.message);
                              let b = encode_all(&mut input, 3).unwrap();
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn param_parser(parts:Vec<&str>,user_parts:Vec<&str>, bhash:&mut HashMap<String,String>){
                              for (b, billion) in parts.iter().zip(user_parts.iter()) {
                                  // println!("{}{}", b, billion);
                                  if b != billion && billion.starts_with(&"<") && billion.ends_with(&">") {
                                      let key = billion.replace("<", "").replace(">", "");
                                      let value = b;
                                      bhash.insert(key.to_string(), value.to_string());
                                  }
                              }
                          }
                          pub fn zen(response:Return,mut stream:&TcpStream,header:Vec<(&str, &str)>,compression:&str,secure_header:&str) {
                              // println!("{}",pathb);
                      // println!("{}",compression);
                      // println!("{}","billionairegreathari");
                      let mut chunks = BufWriter::new(&mut stream);
                      
                      let billionaire = response;
                      // Return { status: 200, message: ["{\"name\":\"billionaireid\"}"], content_type: Some("application/json"), headers: Some([]) }
                      
                      let mut response = format!("HTTP/1.1 {} OK\r\n\
                                  Connection: keep-alive\r\n\
                                  X-Content-Type-Options: nosniff\r\n\
                                  Transfer-Encoding: chunked\r\n",&billionaire.status);
                      
                      if secure_header.len() != 0 {
                        response.push_str(&format!("{}\r\n",secure_header));
                        // println!("{}",billionaire);
                      }
                      // println!("{:?}",header); 
                      if header.len() >= 1 {
                        for header in header.iter(){
                          // println!("{:?}",header);;
                          response.push_str(&format!("{}: {}\r\n",header.0,header.1));
                        }
                      }                               
                      if !response.contains("Content-Security-Policy: "){
                        response.push_str(&format!("Content-Security-Policy: default-src 'self'\r\n"));
                      }
                      if !response.contains("X-Frame-Options: "){
                        response.push_str(&format!("X-Frame-Options: DENY\r\n"));
                      }
                      if !response.contains("Referrer-Policy: "){
                        response.push_str(&format!("Referrer-Policy: no-referrer\r\n"));
                      }
                      if !response.contains("Keep-Alive: "){
                        response.push_str(&format!("Keep-Alive: timeout=10, max=100\r\n"));
                      }
                      if !response.contains("Cache-Control: "){
                        response.push_str(&format!("Cache-Control: public; max-age: 5; stale-while-revalidate: 10\r\n"));
                      }
                      if let Some(res) = &billionaire.content_type {
                      //   println!("{}",res);
                        response.push_str(&format!("Content-Type: {}; charset=UTF-8\r\n",res));
                      }
                      
                      
                      // println!("{}",response);
                      if compression == "" && billionaire.encoding.is_none(){
                        // println!("{}","billionaire");
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        // println!("{:?}",billionaire);
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      if let Some(b) = &billionaire.encoding {
                        if b == "gzip" {
                          response.push_str("Content-Encoding: gzip");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          // println!("{:?}",billionaire);
                          gzip_compress(&billionaire,&mut chunks);
  
                        }
                        else if b == "brotli" {
                          response.push_str("Content-Encoding: br");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          brotli_compress(&billionaire,&mut chunks);
                        }
                        else if compression == "zstd" {
                          response.push_str("Content-Encoding: zstd");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          zstd_compress(&billionaire,&mut chunks);
  
  
                        }
                      }
                      else {
                      if compression == "gzip" {
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      else if compression == "brotli" {
                        response.push_str("Content-Encoding: br");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        brotli_compress(&billionaire,&mut chunks);
  
  
                      }
                      else if compression == "zstd" {
                        response.push_str("Content-Encoding: zstd");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        zstd_compress(&billionaire,&mut chunks);
                      }
                      }
                      chunks.write_all("0\r\n\r\n".as_bytes()).unwrap();
                      chunks.flush().unwrap();     
                      
                      }
                      }
  
                      #block

                      let methodb = response.method();
                      let pathb = response.path();
                      let len = pathb.find("?").or_else(|| Some(pathb.len())).unwrap();
                      let real_path = &pathb[..len];
                      // println!("from {}",pathb);
                      // println!("{}",compression);
                      // println!("billionaire {}",#path);
                      // println!("{}",pathb == #path);
          
                      let parts:Vec<&str> = pathb.split("/").collect();
                      // println!("{:?}",parts);
                      let user_parts:Vec<&str> = #path.split("/").collect();
                      // println!("bbbbbb {}",parts.len() == user_parts.len());
                      // println!("{:?}",user_parts);
          
                      let mut billionaires = HashMap::new();
          
                      if methodb == #method {
                          // println!("{}",methodb);
                          if real_path == #path {
                              // println!("{}","billionairegreat");
                              // println!("{:?}",response);
                              let billionaire = GatesRequest {
                                  body: response.content(),
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query(),
                              };
                              let response = {#fn_name(&billionaire)};
                              if #is_keeper {
                                let gatekeeper = {#function(&billionaire)};
                                // println!("{}","billionairegreat");
                                // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                                if let GateKeeperResponse::Response(b) = gatekeeper {
                                    let mut html = format!("HTTP/1.1 {} {}\r\n",b.status,b.status);
                                    // Return { status: 404, message: "", content_type: Some("application/json"), headers: Some([]), encoding: None }
                                    if let Some(headers) = b.headers {
                                        for header in header.iter() {
                                            html.push_str(&format!("{}: {}\r\n",header.0,header.1));
                                        }
                                    }
                                    if let Some(content_type) = b.content_type {
                                        html.push_str(&format!("Content-Type: {}\r\n",content_type));
                                    }
                                    if let Some(encoding) = b.encoding {
                                        html.push_str(&format!("Content-Encoding: {}\r\n",encoding));
                                    }
                                    html.push_str(&format!("Content-Length: {}\r\n\r\n{}",b.message.len(),b.message));
                                    // println!("{}",html);
                                    let _ = stream.write_all(html.as_bytes()).unwrap();
                                    let _ = stream.flush().unwrap();
                                    // println!("{}","finish");
                                    return; 
                                }
                                if let GateKeeperResponse::Redirect(b,path) = gatekeeper {
                                  let status = match b {
                                    302 => "Temproary Redirect",
                                    301 => "Permanently Moved",
                                    307 => "Temproary Moved",
                                    _ => "OK"
                                  };
                                  let b = format!("HTTP/1.1 {} Permanent Redirect\r\nLocation: {}\r\n\r\n",b,path);
                                  let _ = stream.write_all(b.as_bytes()).unwrap();
                                  let _ = stream.flush().unwrap();
                                  return;
                                }
                                  if let GateKeeperResponse::Next = gatekeeper {
                                    let response = {#fn_name(&billionaire)};
                                    if let GateKeeperResponse::Response(response) = response {
                                      bucket::zen(response,stream,header,compression,secure_header);
                                    }   
                                    else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                      let status = match code {
                                        302 => "Temproary Redirect",
                                        301 => "Permanently Moved",
                                        307 => "Temproary Moved",
                                        _ => "OK"
                                      };
                                      let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                      let _ = stream.write_all(response.as_bytes()).unwrap();
                                      let _ = stream.flush().unwrap();
                                      // println!("{}",response);
                                      return;
                                    }             
                                  }
                              }
                                  // chunks.write_all(billionaire.0.as_bytes());
                                  // // chunks.write_all(&compress);
                                  // // println!("{:?}",b);
                          }
                          else if #path.contains("<") && #path.contains(">")  && !["/favicon.ico"].contains(&pathb) && parts.len() == user_parts.len() {
                              // #b;
                              // println!("{}","billionaires");
                              bucket::param_parser(parts,user_parts,&mut billionaires);
                              // println!("{:?}",billionaires);
                              let billionaire = GatesRequest {
                                  body: response.content(),
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query()
                                  };
                                  if #is_keeper {
                                    let gatekeeper = {#function(&billionaire)};
                                    // println!("{}","billionairegreat");
                                    // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                                    if let GateKeeperResponse::Response(b) = gatekeeper {
                                        let mut html = format!("HTTP/1.1 {} {}\r\n",b.status,b.status);
                                        // Return { status: 404, message: "", content_type: Some("application/json"), headers: Some([]), encoding: None }
                                                                            if secure_header.len() != 0 {
                                      html.push_str(&format!("{}\r\n",secure_header));
                                    }
                                        if let Some(headers) = b.headers {
                                            for header in header.iter() {
                                                html.push_str(&format!("{}: {}\r\n",header.0,header.1));
                                            }
                                        }
                                        if let Some(content_type) = b.content_type {
                                            html.push_str(&format!("Content-Type: {}\r\n",content_type));
                                        }
                                        if let Some(encoding) = b.encoding {
                                            html.push_str(&format!("Content-Encoding: {}\r\n",encoding));
                                        }
                                        html.push_str(&format!("Content-Length: {}\r\n\r\n{}",b.message.len(),b.message));
                                        // println!("{}",html);
                                        let _ = stream.write_all(html.as_bytes()).unwrap();
                                        let _ = stream.flush().unwrap();
                                        // println!("{}","finish");
                                        return; 
                                    }
                                    else if let GateKeeperResponse::Redirect(b,path) = gatekeeper {
                                  let mut b = format!("HTTP/1.1 {} Permanent Redirect\r\n",b);
                                  if secure_header.len() != 0 {
                                    b.push_str(&format!("{}\r\n",secure_header));
                                  }
                                  b.push_str(&format!("Location: {}\r\n\r\n",path));
                                      let _ = stream.write_all(b.as_bytes()).unwrap();
                                      let _ = stream.flush().unwrap();
                                      return;
                                  }
                                  else if let GateKeeperResponse::Next = gatekeeper {
                                    let response = {#fn_name(&billionaire)};
                                    if let GateKeeperResponse::Response(response) = response {
                                      bucket::zen(response,stream,header,compression,secure_header);
                                    }   
                                    else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                      let status = match code {
                                        302 => "Temproary Redirect",
                                        301 => "Permanently Moved",
                                        307 => "Temproary Moved",
                                        _ => "OK"
                                      };
                                      let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                      let _ = stream.write_all(response.as_bytes()).unwrap();
                                      let _ = stream.flush().unwrap();
                                      // println!("{}",response);
                                      return;
                                    }             
                                  }
                                  }
                          }
                      }
                  }
              }
              .into()
          }
          "DELETE" => {
              quote! {
                  // use nous::HashMap;
                  // use nous::Response;
                  // // use nous::TcpStream;
                  // use nous::GatesRequest;
                  // use std::io::BufWriter;
                  // use nous::flate2::write::GzEncoder;
                  // use nous::flate2::Compression;
                  // use nous::brotli::CompressorWriter;
                  // use std::io::Cursor;
                  // use nous::zstd::encode_all;
                  // use nous::Write;
                  // use nous::GateKeeperResponse;
                  // use nous::TcpStream;
                  // use nous::Response;
                  // use nous::HashMap;
                  // use nous::Write;
                  pub fn #fn_name(mut stream:&TcpStream,response:&Response,compression:&str,secure_header:&str,header:Vec<(&str, &str)>) {
                      mod bucket {
                          use nous::HashMap;
                          use nous::Response;
                          use nous::TcpStream;
                          use nous::GatesRequest;
                          use std::io::BufWriter;
                          use nous::flate2::write::GzEncoder;
                          use nous::flate2::Compression;
                          use nous::brotli::CompressorWriter;
                          use std::io::Cursor;
                          use nous::zstd::encode_all;
                          use nous::Write;
                          // use nous::GateKeeperResponse;
                          // use crate::#function;
                          // use crate::#fn_name;
                          use nous::Return;
                          pub fn gzip_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = GzEncoder::new(Vec::new(), Compression::default());
                              b.write_all(&billionaire.message.as_bytes()).unwrap();
                              let b = b.finish();
                              if let Ok(response) = b {
                                  for b in response.chunks(6) {
                                      // hexa of chunk\r\n
                                      // chunk\r\n
                                      let billionaire = format!("{:X}\r\n", b.len());
                                      // println!("{}",billionaire);
                                      let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                      let _ = stream.write_all(b).unwrap();
                                      // println!("{:?}",b);
                                      let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                  }
                              }
                          }
                          pub fn brotli_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = Vec::new();
                              {
                                  let mut b = CompressorWriter::new(&mut b, 4096, 11, 22);
                                  b.write_all(billionaire.message.as_bytes()).unwrap();
                                  b.flush().unwrap();
                              }
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn zstd_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut input = Cursor::new(&billionaire.message);
                              let b = encode_all(&mut input, 3).unwrap();
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn param_parser(parts:Vec<&str>,user_parts:Vec<&str>, bhash:&mut HashMap<String,String>){
                              for (b, billion) in parts.iter().zip(user_parts.iter()) {
                                  // println!("{}{}", b, billion);
                                  if b != billion && billion.starts_with(&"<") && billion.ends_with(&">") {
                                      let key = billion.replace("<", "").replace(">", "");
                                      let value = b;
                                      bhash.insert(key.to_string(), value.to_string());
                                  }
                              }
                          }
                          pub fn zen(response:Return,mut stream:&TcpStream,header:Vec<(&str, &str)>,compression:&str,secure_header:&str) {
                              // println!("{}",pathb);
                      // println!("{}",compression);
                      // println!("{}","billionairegreathari");
                      let mut chunks = BufWriter::new(&mut stream);
                      
                      let billionaire = response;
                      // Return { status: 200, message: ["{\"name\":\"billionaireid\"}"], content_type: Some("application/json"), headers: Some([]) }
                      
                      let mut response = format!("HTTP/1.1 {} OK\r\n\
                                  Connection: keep-alive\r\n\
                                  X-Content-Type-Options: nosniff\r\n\
                                  Transfer-Encoding: chunked\r\n",&billionaire.status);
                      
                      if secure_header.len() != 0 {
                        response.push_str(&format!("{}\r\n",secure_header));
                        // println!("{}",billionaire);
                      }
                      // println!("{:?}",header); 
                      if header.len() >= 1 {
                        for header in header.iter(){
                          // println!("{:?}",header);;
                          response.push_str(&format!("{}: {}\r\n",header.0,header.1));
                        }
                      }                               
                      if !response.contains("Content-Security-Policy: "){
                        response.push_str(&format!("Content-Security-Policy: default-src 'self'\r\n"));
                      }
                      if !response.contains("X-Frame-Options: "){
                        response.push_str(&format!("X-Frame-Options: DENY\r\n"));
                      }
                      if !response.contains("Referrer-Policy: "){
                        response.push_str(&format!("Referrer-Policy: no-referrer\r\n"));
                      }
                      if !response.contains("Keep-Alive: "){
                        response.push_str(&format!("Keep-Alive: timeout=10, max=100\r\n"));
                      }
                      if !response.contains("Cache-Control: "){
                        response.push_str(&format!("Cache-Control: public; max-age: 5; stale-while-revalidate: 10\r\n"));
                      }
                      if let Some(res) = &billionaire.content_type {
                      //   println!("{}",res);
                        response.push_str(&format!("Content-Type: {}; charset=UTF-8\r\n",res));
                      }
                      
                      
                      // println!("{}",response);
                      if compression == "" && billionaire.encoding.is_none() {
                        // println!("{}","billionaire");
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        // println!("{:?}",billionaire);
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      if let Some(b) = &billionaire.encoding {
                        if b == "gzip" {
                          response.push_str("Content-Encoding: gzip");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          // println!("{:?}",billionaire);
                          gzip_compress(&billionaire,&mut chunks);
  
                        }
                        else if b == "brotli" {
                          response.push_str("Content-Encoding: br");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          brotli_compress(&billionaire,&mut chunks);
                        }
                        else if compression == "zstd" {
                          response.push_str("Content-Encoding: zstd");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          zstd_compress(&billionaire,&mut chunks);
  
  
                        }
                      }
                      else {
                      if compression == "gzip" {
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      else if compression == "brotli" {
                        response.push_str("Content-Encoding: br");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        brotli_compress(&billionaire,&mut chunks);
  
  
                      }
                      else if compression == "zstd" {
                        response.push_str("Content-Encoding: zstd");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        zstd_compress(&billionaire,&mut chunks);
                      }
                      }
                      chunks.write_all("0\r\n\r\n".as_bytes()).unwrap();
                      chunks.flush().unwrap();     
                      
                      }
                      }
  
                      #block

                      let methodb = response.method();
                      let pathb = response.path();
                      let len = pathb.find("?").or_else(|| Some(pathb.len())).unwrap();
                      let real_path = &pathb[..len];
                      // println!("from {}",pathb);
                      // println!("{}",compression);
                      // println!("billionaire {}",#path);
                      // println!("{}",pathb == #path);
          
                      let parts:Vec<&str> = pathb.split("/").collect();
                      // println!("{:?}",parts);
                      let user_parts:Vec<&str> = #path.split("/").collect();
                      // println!("bbbbbb {}",parts.len() == user_parts.len());
                      // println!("{:?}",user_parts);
          
                      let mut billionaires = HashMap::new();
          
                      if methodb == #method {
                          // println!("{}",methodb);
                          if real_path == #path {
                              // println!("{}","billionairegreat");
                              // println!("{:?}",response);
                              let billionaire = GatesRequest {
                                  body: response.content(),
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query(),
                              };
                              let response = {#fn_name(&billionaire)};
                              if #is_keeper {
                                let gatekeeper = {#function(&billionaire)};
                                // println!("{}","billionairegreat");
                                // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                                if let GateKeeperResponse::Response(b) = gatekeeper {
                                    let mut html = format!("HTTP/1.1 {} {}\r\n",b.status,b.status);
                                    // Return { status: 404, message: "", content_type: Some("application/json"), headers: Some([]), encoding: None }
                                    if let Some(headers) = b.headers {
                                        for header in header.iter() {
                                            html.push_str(&format!("{}: {}\r\n",header.0,header.1));
                                        }
                                    }
                                    if let Some(content_type) = b.content_type {
                                        html.push_str(&format!("Content-Type: {}\r\n",content_type));
                                    }
                                    if let Some(encoding) = b.encoding {
                                        html.push_str(&format!("Content-Encoding: {}\r\n",encoding));
                                    }
                                    html.push_str(&format!("Content-Length: {}\r\n\r\n{}",b.message.len(),b.message));
                                    // println!("{}",html);
                                    let _ = stream.write_all(html.as_bytes()).unwrap();
                                    let _ = stream.flush().unwrap();
                                    // println!("{}","finish");
                                    return; 
                                }
                                if let GateKeeperResponse::Redirect(b,path) = gatekeeper {
                                  let status = match b {
                                    302 => "Temproary Redirect",
                                    301 => "Permanently Moved",
                                    307 => "Temproary Moved",
                                    _ => "OK"
                                  };
                                  let b = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",b,status,path);
                                  let _ = stream.write_all(b.as_bytes()).unwrap();
                                  let _ = stream.flush().unwrap();
                                  return;
                                }
                                  if let GateKeeperResponse::Next = gatekeeper {
                                    let response = {#fn_name(&billionaire)};
                                    if let GateKeeperResponse::Response(response) = response {
                                      bucket::zen(response,stream,header,compression,secure_header);
                                    }   
                                    else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                      let status = match code {
                                        302 => "Temproary Redirect",
                                        301 => "Permanently Moved",
                                        307 => "Temproary Moved",
                                        _ => "OK"
                                      };
                                      let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                      let _ = stream.write_all(response.as_bytes()).unwrap();
                                      let _ = stream.flush().unwrap();
                                      // println!("{}",response);
                                      return;
                                    }             
                                  }
                              }
                                  // chunks.write_all(billionaire.0.as_bytes());
                                  // // chunks.write_all(&compress);
                                  // // println!("{:?}",b);
                          }
                          else if #path.contains("<") && #path.contains(">")  && !["/favicon.ico"].contains(&pathb) && parts.len() == user_parts.len() {
                              // #b;
                              // println!("{}","billionaires");
                              bucket::param_parser(parts,user_parts,&mut billionaires);
                              // println!("{:?}",billionaires);
                              let billionaire = GatesRequest {
                                  body: response.content(),
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query()
                                  };
                                  if #is_keeper {
                                    let gatekeeper = {#function(&billionaire)};
                                    // println!("{}","billionairegreat");
                                    // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                                    if let GateKeeperResponse::Response(b) = gatekeeper {
                                        let mut html = format!("HTTP/1.1 {} {}\r\n",b.status,b.status);
                                        // Return { status: 404, message: "", content_type: Some("application/json"), headers: Some([]), encoding: None }
                                                                            if secure_header.len() != 0 {
                                      html.push_str(&format!("{}\r\n",secure_header));
                                    }
                                        if let Some(headers) = b.headers {
                                            for header in header.iter() {
                                                html.push_str(&format!("{}: {}\r\n",header.0,header.1));
                                            }
                                        }
                                        if let Some(content_type) = b.content_type {
                                            html.push_str(&format!("Content-Type: {}\r\n",content_type));
                                        }
                                        if let Some(encoding) = b.encoding {
                                            html.push_str(&format!("Content-Encoding: {}\r\n",encoding));
                                        }
                                        html.push_str(&format!("Content-Length: {}\r\n\r\n{}",b.message.len(),b.message));
                                        // println!("{}",html);
                                        let _ = stream.write_all(html.as_bytes()).unwrap();
                                        let _ = stream.flush().unwrap();
                                        // println!("{}","finish");
                                        return; 
                                    }
                                    if let GateKeeperResponse::Redirect(b,path) = gatekeeper {
                                  let mut b = format!("HTTP/1.1 {} Permanent Redirect\r\n",b);
                                  if secure_header.len() != 0 {
                                    b.push_str(&format!("{}\r\n",secure_header));
                                  }
                                  b.push_str(&format!("Location: {}\r\n\r\n",path));
                                      let _ = stream.write_all(b.as_bytes()).unwrap();
                                      let _ = stream.flush().unwrap();
                                      return;
                                    }
                                  if let GateKeeperResponse::Next = gatekeeper {
                                    let response = {#fn_name(&billionaire)};
                                    if let GateKeeperResponse::Response(response) = response {
                                      bucket::zen(response,stream,header,compression,secure_header);
                                    }   
                                    else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                      let status = match code {
                                        302 => "Temproary Redirect",
                                        301 => "Permanently Moved",
                                        307 => "Temproary Moved",
                                        _ => "OK"
                                      };
                                      let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                      let _ = stream.write_all(response.as_bytes()).unwrap();
                                      let _ = stream.flush().unwrap();
                                      // println!("{}",response);
                                      return;
                                    }             
                                  }
                                  }
                          }
                      }
                  }
              }
              .into()
          }
          "text/event-stream" => {
            quote! {
              // use nous::HashMap;
              // use nous::Response;
              // use nous::TcpStream;
              // use nous::GatesRequest;
              // use std::io::BufWriter;
              // use nous::flate2::write::GzEncoder;
              // use nous::flate2::Compression;
              // use nous::brotli::CompressorWriter;
              // use std::io::Cursor;
              // use nous::zstd::encode_all;
              // use nous::Write;
              // use nous::GateKeeperResponse;

              pub fn #fn_name(mut stream:&TcpStream,response:&Response,compression:&str,secure_header:&str,header:Vec<(&str, &str)>) {
                  mod bucket {
                      use nous::HashMap;
                      use nous::Response;
                      use nous::TcpStream;
                      use nous::GatesRequest;
                      use std::io::BufWriter;
                      use nous::Write;
                      // use nous::GateKeeperResponse;
                      // use crate::#function;
                      // use crate::#fn_name;
                      use nous::Return;
                      pub fn param_parser(parts:Vec<&str>,user_parts:Vec<&str>, bhash:&mut HashMap<String,String>){
                          for (b, billion) in parts.iter().zip(user_parts.iter()) {
                              // println!("{}{}", b, billion);
                              if b != billion && billion.starts_with(&"<") && billion.ends_with(&">") {
                                  let key = billion.replace("<", "").replace(">", "");
                                  let value = b;
                                  bhash.insert(key.to_string(), value.to_string());
                              }
                          }
                      }
                      pub fn zen(response:Return,mut stream:&TcpStream,header:Vec<(&str, &str)>,compression:&str,secure_header:&str) {
                          // println!("{}",pathb);
                  // println!("{}",compression);
                  // println!("{}","billionairegreathari");
                  let mut chunks = BufWriter::new(&mut stream);
                  
                  let billionaire = response;
                  // Return { status: 200, message: ["{\"name\":\"billionaireid\"}"], content_type: Some("application/json"), headers: Some([]) }
                  
                  let mut response = format!("HTTP/1.1 {} OK\r\n\
                          Connection: keep-alive\r\n\
                          X-Content-Type-Options: nosniff\r\n\
                          Content-Type: text/event-stream\r\n",&billionaire.status);
                              // Transfer-Encoding: chunked\r\n
                  
                  if secure_header.len() != 0 {
                      response.push_str(&format!("{}\r\n",secure_header));
                                // println!("{}",billionaire);
                  }
                  // println!("{:?}",header); 
                  if header.len() >= 1 {
                    for header in header.iter(){
                      // println!("{:?}",header);;
                        response.push_str(&format!("{}: {}\r\n",header.0,header.1));
                    }
                  }                               
                  if !response.contains("Content-Security-Policy: "){
                    response.push_str(&format!("Content-Security-Policy: default-src 'self'\r\n"));
                  }
                  if !response.contains("X-Frame-Options: "){
                    response.push_str(&format!("X-Frame-Options: DENY\r\n"));
                  }
                  if !response.contains("Referrer-Policy: "){
                    response.push_str(&format!("Referrer-Policy: no-referrer\r\n"));
                  }
                  if !response.contains("Keep-Alive: "){
                    response.push_str(&format!("Keep-Alive: timeout=10, max=100\r\n"));
                  }
                  if !response.contains("Cache-Control: "){
                    response.push_str(&format!("Cache-Control: public; max-age: 5; stale-while-revalidate: 10\r\n"));
                  }
                  // if let Some(res) = &billionaire.content_type {
                  // //   println!("{}",res);
                  //   response.push_str(&format!("Content-Type: {}; charset=UTF-8\r\n",res));
                  // }
                  
                  
                  // println!("{}",response);
                  
  
                  let b: Vec<&str> = billionaire.message.split_whitespace().collect();
                  // println!("{:?}", b);
                  response.push_str(&format!("\r\n"));
                  // println!("{:?}",response);
                  chunks.write_all(response.as_bytes()).unwrap();
              
                  for b in b {
                      let billionaire = format!("data: {}\n\n", b);
                      // println!("{:?}",billionaire);
                      chunks.write_all(&billionaire.as_bytes()).unwrap();
                      chunks.flush().unwrap();
                  }
                  }
                  }
  
                  #block

                  let methodb = response.method();
                  let pathb = response.path();
                  let len = pathb.find("?").or_else(|| Some(pathb.len())).unwrap();
                  let real_path = &pathb[..len];
                  // println!("from {}",pathb);
                  // println!("{}",compression);
                  // println!("billionaire {}",#path);
                  // println!("{}",pathb == #path);
      
                  // println!("{}",real_path);
  
                  let parts:Vec<&str> = pathb.split("/").collect();
                  // println!("{:?}",parts);
                  let user_parts:Vec<&str> = #path.split("/").collect();
                  // println!("bbbbbb {}",parts.len() == user_parts.len());
                  // println!("{:?}",user_parts);
      
                  let mut billionaires = HashMap::new();
      
                  let headerb = response.header();
                  // println!("{:?}",header);
                  let headerb = headerb.get("Accept").unwrap().trim();
                  // println!("{}",headerb);
                  if "GET" == methodb && "text/event-stream" == headerb {
                      // println!("==============================={}",methodb);
                      // println!("{}",b);
                      if real_path == #path {
                          // println!("{}","billionairegreat");
                          // println!("{:?}",response.query());
                          let billionaire = GatesRequest {
                              body: "",
                              cookies: &response.cookie(),
                              headers: response.header(),
                              path: response.path(),
                              method: response.method(),
                              param: billionaires,
                              query: response.query(),
                          };
                          // println!("{:?}",billionaire);
                          let response = {#fn_name(&billionaire)};

                          // println!("{}","peek");
                          if #is_keeper {
                            let gatekeeper = {#function(&billionaire)};

                            // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                            if let GateKeeperResponse::Response(b) = gatekeeper {
                                let mut html = format!("HTTP/1.1 {} {}\r\n",b.status,b.status);
                                // Return { status: 404, message: "", content_type: Some("application/json"), headers: Some([]), encoding: None }
                                if let Some(headers) = b.headers {
                                    for header in header.iter() {
                                        html.push_str(&format!("{}: {}\r\n",header.0,header.1));
                                    }
                                }
                                if let Some(content_type) = b.content_type {
                                    html.push_str(&format!("Content-Type: {}\r\n",content_type));
                                }
                                if let Some(encoding) = b.encoding {
                                    html.push_str(&format!("Content-Encoding: {}\r\n",encoding));
                                }
                                html.push_str(&format!("Content-Length: {}\r\n\r\n{}",b.message.len(),b.message));
                                // println!("{}",html);
                                let _ = stream.write_all(html.as_bytes()).unwrap();
                                let _ = stream.flush().unwrap();
                                // println!("{}","finish");
                                return; 
                            }
                            if let GateKeeperResponse::Redirect(b,path) = gatekeeper {
                                    let status = match b {
                                      302 => "Temproary Redirect",
                                      301 => "Permanently Moved",
                                      307 => "Temproary Moved",
                                      _ => "OK"
                                    };
                              let b = format!("HTTP/1.1 {} Permanent Redirect\r\nLocation: {}\r\n\r\n",b,path);
                              let _ = stream.write_all(b.as_bytes()).unwrap();
                              let _ = stream.flush().unwrap();
                              return;
                            }
                            if let GateKeeperResponse::Next = gatekeeper {
                              // println!("{}","billionaire");
                              let response = {#fn_name(&billionaire)};
                              // println!("{:?}",response);
                              if let GateKeeperResponse::Response(response) = response {
                                bucket::zen(response,stream,header,compression,secure_header);
                              }   
                              else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                let status = match code {
                                  302 => "Temproary Redirect",
                                  301 => "Permanently Moved",
                                  307 => "Temproary Moved",
                                  _ => "OK"
                                };
                                let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                let _ = stream.write_all(response.as_bytes()).unwrap();
                                let _ = stream.flush().unwrap();
                                // println!("{}",response);
                                return;
                              }             
                            }
                          }
                              // chunks.write_all(billionaire.0.as_bytes());
                              // // chunks.write_all(&compress);
                              // // println!("{:?}",b);
                      }
                      else if #path.contains("<") && #path.contains(">")  && !["/favicon.ico"].contains(&pathb) && parts.len() == user_parts.len() {
                          // #b;
                          // println!("{}","billionaires");
                          bucket::param_parser(parts,user_parts,&mut billionaires);
                          // println!("{:?}",billionaires);
                          let billionaire = GatesRequest {
                              body: "",
                              cookies: &response.cookie(),
                              headers: response.header(),
                              path: response.path(),
                              method: response.method(),
                              param: billionaires,
                              query: response.query()
                          };
                              // handle this b
                          // let b = {#function(&billionaire)} ;
                          if #is_keeper {
                            let gatekeeper = {#function(&billionaire)};

                            // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                            if let GateKeeperResponse::Response(b) = gatekeeper {
                                let mut html = format!("HTTP/1.1 {} {}\r\n",b.status,b.status);
                                // Return { status: 404, message: "", content_type: Some("application/json"), headers: Some([]), encoding: None }
                                if let Some(headers) = b.headers {
                                    for header in header.iter() {
                                        html.push_str(&format!("{}: {}\r\n",header.0,header.1));
                                    }
                                }
                                if let Some(content_type) = b.content_type {
                                    html.push_str(&format!("Content-Type: {}\r\n",content_type));
                                }
                                if let Some(encoding) = b.encoding {
                                    html.push_str(&format!("Content-Encoding: {}\r\n",encoding));
                                }
                                html.push_str(&format!("Content-Length: {}\r\n\r\n{}",b.message.len(),b.message));
                                // println!("{}",html);
                                let _ = stream.write_all(html.as_bytes()).unwrap();
                                let _ = stream.flush().unwrap();
                                // println!("{}","finish");
                                return; 
                            }
                            if let GateKeeperResponse::Redirect(b,path) = gatekeeper {
                                    let status = match b {
                                      302 => "Temproary Redirect",
                                      301 => "Permanently Moved",
                                      307 => "Temproary Moved",
                                      _ => "OK"
                                    };
                              let b = format!("HTTP/1.1 {} Permanent Redirect\r\nLocation: {}\r\n\r\n",b,path);
                              let _ = stream.write_all(b.as_bytes()).unwrap();
                              let _ = stream.flush().unwrap();
                              return;
                            }
                            if let GateKeeperResponse::Next = gatekeeper {
                                let response = {#fn_name(&billionaire)};
                                if let GateKeeperResponse::Response(response) = response {
                                  bucket::zen(response,stream,header,compression,secure_header);
                                }   
                                else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                  let status = match code {
                                    302 => "Temproary Redirect",
                                    301 => "Permanently Moved",
                                    307 => "Temproary Moved",
                                    _ => "OK"
                                  };
                                  let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                  let _ = stream.write_all(response.as_bytes()).unwrap();
                                  let _ = stream.flush().unwrap();
                                  // println!("{}",response);
                                  return;
                                }             
                            }
                          }
                      }
                  }
              }
          }
          .into()
          }
          _ => {
              quote! {
                  use nous::TcpStream;
                  use nous::Response;
                  use nous::HashMap;
                  use nous::Write;
                  //                   use nous::HashMap;
                  // use nous::Response;
                  // // use nous::TcpStream;
                  // use nous::GatesRequest;
                  // use std::io::BufWriter;
                  // use nous::flate2::write::GzEncoder;
                  // use nous::flate2::Compression;
                  // use nous::brotli::CompressorWriter;
                  // use std::io::Cursor;
                  // use nous::zstd::encode_all;
                  // use nous::Write;
                  // use nous::GateKeeperResponse;
                  // use nous::TcpStream;
                  // use nous::Response;
                  // use nous::HashMap;
                  // use nous::Write;
                  pub fn #fn_name(mut stream:&TcpStream,response:&Response,compression:&str,secure_header:&str,header:Vec<(&str, &str)>) {
                      mod bucket {
                          use nous::HashMap;
                          use nous::Response;
                          use nous::TcpStream;
                          use nous::GatesRequest;
                          use std::io::BufWriter;
                          use nous::flate2::write::GzEncoder;
                          use nous::flate2::Compression;
                          use nous::brotli::CompressorWriter;
                          use std::io::Cursor;
                          use nous::zstd::encode_all;
                          use nous::Write;
                          // use nous::GateKeeperResponse;
                          // use crate::#function;
                          // use crate::#fn_name;
                          use nous::Return;
                          pub fn gzip_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = GzEncoder::new(Vec::new(), Compression::default());
                              b.write_all(&billionaire.message.as_bytes()).unwrap();
                              let b = b.finish();
                              if let Ok(response) = b {
                                  for b in response.chunks(6) {
                                      // hexa of chunk\r\n
                                      // chunk\r\n
                                      let billionaire = format!("{:X}\r\n", b.len());
                                      // println!("{}",billionaire);
                                      let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                      let _ = stream.write_all(b).unwrap();
                                      // println!("{:?}",b);
                                      let _ = stream.write_all("\r\n".as_bytes()).unwrap()
                                  }
                              }
                          }
                          pub fn brotli_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = Vec::new();
                              {
                                  let mut b = CompressorWriter::new(&mut b, 4096, 11, 22);
                                  b.write_all(billionaire.message.as_bytes()).unwrap();
                                  b.flush();
                              }
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn zstd_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut input = Cursor::new(&billionaire.message);
                              let b = encode_all(&mut input, 3).unwrap();
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn param_parser(parts:Vec<&str>,user_parts:Vec<&str>, bhash:&mut HashMap<String,String>){
                              for (b, billion) in parts.iter().zip(user_parts.iter()) {
                                  // println!("{}{}", b, billion);
                                  if b != billion && billion.starts_with(&"<") && billion.ends_with(&">") {
                                      let key = billion.replace("<", "").replace(">", "");
                                      let value = b;
                                      bhash.insert(key.to_string(), value.to_string());
                                  }
                              }
                          }
                          pub fn zen(response:Return,mut stream:&TcpStream,header:Vec<(&str, &str)>,compression:&str,secure_header:&str) {
                              // println!("{}",pathb);
                      // println!("{}",compression);
                      // println!("{}","billionairegreathari");
                      let mut chunks = BufWriter::new(&mut stream);
                      
                      let billionaire = response;
                      // Return { status: 200, message: ["{\"name\":\"billionaireid\"}"], content_type: Some("application/json"), headers: Some([]) }
                      
                      let mut response = format!("HTTP/1.1 {} OK\r\n\
                                  Connection: keep-alive\r\n\
                                  X-Content-Type-Options: nosniff\r\n\
                                  Transfer-Encoding: chunked\r\n",&billionaire.status);
                      
                                            if secure_header.len() != 0 {
                        response.push_str(&format!("{}\r\n",secure_header));
                        // println!("{}",billionaire);
                      }
                      // println!("{:?}",header); 
                      if header.len() >= 1 {
                        for header in header.iter(){
                          // println!("{:?}",header);;
                          response.push_str(&format!("{}: {}\r\n",header.0,header.1));
                        }
                      }                               
                      if !response.contains("Content-Security-Policy: "){
                        response.push_str(&format!("Content-Security-Policy: default-src 'self'\r\n"));
                      }
                      if !response.contains("X-Frame-Options: "){
                        response.push_str(&format!("X-Frame-Options: DENY\r\n"));
                      }
                      if !response.contains("Referrer-Policy: "){
                        response.push_str(&format!("Referrer-Policy: no-referrer\r\n"));
                      }
                      if !response.contains("Keep-Alive: "){
                        response.push_str(&format!("Keep-Alive: timeout=10, max=100\r\n"));
                      }
                      if !response.contains("Cache-Control: "){
                        response.push_str(&format!("Cache-Control: public; max-age: 5; stale-while-revalidate: 10\r\n"));
                      }
                      if let Some(res) = &billionaire.content_type {
                      //   println!("{}",res);
                        response.push_str(&format!("Content-Type: {}; charset=UTF-8\r\n",res));
                      }
                      
                      
                      // println!("{}",response);
                      
                      if let Some(b) = &billionaire.encoding {
                        if b == "gzip" {
                          response.push_str("Content-Encoding: gzip");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          // println!("{:?}",billionaire);
                          gzip_compress(&billionaire,&mut chunks);
  
                        }
                        else if b == "brotli" {
                          response.push_str("Content-Encoding: br");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          brotli_compress(&billionaire,&mut chunks);
                        }
                        else if compression == "zstd" {
                          response.push_str("Content-Encoding: zstd");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          zstd_compress(&billionaire,&mut chunks);
  
  
                        }
                      }
                      else {
                      if compression == "gzip" {
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      else if compression == "brotli" {
                        response.push_str("Content-Encoding: br");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        brotli_compress(&billionaire,&mut chunks);
  
  
                      }
                      else if compression == "zstd" {
                        response.push_str("Content-Encoding: zstd");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        zstd_compress(&billionaire,&mut chunks);
                      }
                      }
                      chunks.write_all("0\r\n\r\n".as_bytes()).unwrap();
                      chunks.flush().unwrap();     
                      
                      }
                      }
  
                      #block

                      let methodb = response.method();
                      let pathb = response.path();
                      let len = pathb.find("?").or_else(|| Some(pathb.len())).unwrap();
                      let real_path = &pathb[..len];
                      // println!("from {}",pathb);
                      // println!("{}",compression);
                      // println!("billionaire {}",#path);
                      // println!("{}",pathb == #path);
          
                      let parts:Vec<&str> = pathb.split("/").collect();
                      // println!("{:?}",parts);
                      let user_parts:Vec<&str> = #path.split("/").collect();
                      // println!("bbbbbb {}",parts.len() == user_parts.len());
                      // println!("{:?}",user_parts);
          
                      let mut billionaires = HashMap::new();
          
                      if methodb == #method {
                          // println!("{}",methodb);
                          if real_path == #path {
                              // println!("{}","billionairegreat");
                              // println!("{:?}",response);
                              let billionaire = GatesRequest {
                                  body: "",
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query(),
                              };
                              let response = {#fn_name(&billionaire)};
                              if #is_keeper {
                                let gatekeeper = {#function(&billionaire)};
                                // println!("{}","billionairegreat");
                                // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                                if let GateKeeperResponse::Response(b) = gatekeeper {
                                    let mut html = format!("HTTP/1.1 {} {}\r\n",b.status,b.status);
                                    // Return { status: 404, message: "", content_type: Some("application/json"), headers: Some([]), encoding: None }
                                    if let Some(headers) = b.headers {
                                        for header in header.iter() {
                                            html.push_str(&format!("{}: {}\r\n",header.0,header.1));
                                        }
                                    }
                                    if let Some(content_type) = b.content_type {
                                        html.push_str(&format!("Content-Type: {}\r\n",content_type));
                                    }
                                    if let Some(encoding) = b.encoding {
                                        html.push_str(&format!("Content-Encoding: {}\r\n",encoding));
                                    }
                                    html.push_str(&format!("Content-Length: {}\r\n\r\n{}",b.message.len(),b.message));
                                    // println!("{}",html);
                                    let _ = stream.write_all(html.as_bytes()).unwrap();
                                    let _ = stream.flush().unwrap();
                                    // println!("{}","finish");
                                    return; 
                                }
                                else if let GateKeeperResponse::Redirect(b,path) = gatekeeper {
                                    let status = match code {
                                      302 => "Temproary Redirect",
                                      301 => "Permanently Moved",
                                      307 => "Temproary Moved",
                                      _ => "OK"
                                    };
                                  let b = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",b,status,path);
                                  let _ = stream.write_all(b.as_bytes()).unwrap();
                                  let _ = stream.flush().unwrap();
                                  return;
                                }
                                else if let GateKeeperResponse::Next = gatekeeper {
                                    let response = {#fn_name(&billionaire)};
                                    if let GateKeeperResponse::Response(response) = response {
                                      bucket::zen(response,stream,header,compression,secure_header);
                                    }   
                                    else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                      let status = match code {
                                        302 => "Temproary Redirect",
                                        301 => "Permanently Moved",
                                        307 => "Temproary Moved",
                                        _ => "OK"
                                      };
                                      let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                      let _ = stream.write_all(response.as_bytes()).unwrap();
                                      let _ = stream.flush().unwrap();
                                      // println!("{}",response);
                                      return;
                                    }             
                                  }
                              }
                                  // chunks.write_all(billionaire.0.as_bytes());
                                  // // chunks.write_all(&compress);
                                  // // println!("{:?}",b);
                          }
                          else if #path.contains("<") && #path.contains(">")  && !["/favicon.ico"].contains(&pathb) && parts.len() == user_parts.len() {
                              // #b;
                              // println!("{}","billionaires");
                              bucket::param_parser(parts,user_parts,&mut billionaires);
                              // println!("{:?}",billionaires);
                              let billionaire = GatesRequest {
                                  body: "",
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query()
                                  };
                              if #is_keeper {
                                let gatekeeper = {#function(&billionaire)};
                                // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                                if let Err(b) = gatekeeper {
                                    let mut html = format!("HTTP/1.1 {} {}\r\n",b.status,b.status);
                                    // Return { status: 404, message: "", content_type: Some("application/json"), headers: Some([]), encoding: None }
                                    if let Some(headers) = b.headers {
                                        for header in header.iter() {
                                            html.push_str(&format!("{}: {}\r\n",header.0,header.1));
                                        }
                                    }
                                    if let Some(content_type) = b.content_type {
                                        html.push_str(&format!("Content-Type: {}\r\n",content_type));
                                    }
                                    if let Some(encoding) = b.encoding {
                                        html.push_str(&format!("Content-Encoding: {}\r\n",encoding));
                                    }
                                    html.push_str(&format!("Content-Length: {}\r\n\r\n{}",b.message.len(),b.message));
                                    // println!("{}",html);
                                    let _ = stream.write_all(html.as_bytes()).unwrap();
                                    let _ = stream.flush().unwrap();
                                    // println!("{}","finish");
                                    return 
                                }
                              };
                              let response = {#fn_name(&billionaire)};
                              bucket::zen(response,stream,header,compression,secure_header);
                          }
                      }
                  }
              }
              .into()
          }
      }
      }
      false => {
        // println!("111111111111111111111111111111111111111 {}",method);
        match method.as_str() {
          "GET" => {
              quote! {
                  // use nous::HashMap;
                  // use nous::Response;
                  // // use nous::TcpStream;
                  // use nous::GatesRequest;
                  // use std::io::BufWriter;
                  // use nous::flate2::write::GzEncoder;
                  // use nous::flate2::Compression;
                  // use nous::brotli::CompressorWriter;
                  // use std::io::Cursor;
                  // use nous::zstd::encode_all;
                  // use nous::Write;
                  // use nous::GateKeeperResponse;                  use nous::HashMap;
                  // use nous::Response;
                  // // use nous::TcpStream;
                  // use nous::GatesRequest;
                  // use std::io::BufWriter;
                  // use nous::flate2::write::GzEncoder;
                  // use nous::flate2::Compression;
                  // use nous::brotli::CompressorWriter;
                  // use std::io::Cursor;
                  // use nous::zstd::encode_all;
                  // use nous::Write;
                  // use nous::GateKeeperResponse;
                  use nous::TcpStream;
                  use nous::Response;
                  use nous::HashMap;
                  use nous::Write;
                  // use nous::GateKeeperResponse;
                  pub fn #fn_name(mut stream:&TcpStream,response:&Response,compression:&str,secure_header:&str,header:Vec<(&str, &str)>) {
                      mod bucket {
                          use nous::HashMap;
                          use nous::Response;
                          use nous::TcpStream;
                          use nous::GatesRequest;
                          use std::io::BufWriter;
                          use nous::flate2::write::GzEncoder;
                          use nous::flate2::Compression;
                          use nous::brotli::CompressorWriter;
                          use std::io::Cursor;
                          use nous::zstd::encode_all;
                          use nous::Write;
                          // use nous::GateKeeperResponse;
                          use nous::Return;
  
                          pub fn gzip_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = GzEncoder::new(Vec::new(), Compression::default());
                              b.write_all(&billionaire.message.as_bytes()).unwrap();
                              let b = b.finish();
                              if let Ok(response) = b {
                                  for b in response.chunks(6) {
                                      // hexa of chunk\r\n
                                      // chunk\r\n
                                      let billionaire = format!("{:X}\r\n", b.len());
                                      // println!("{}",billionaire);
                                      let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                      let _ = stream.write_all(b).unwrap();
                                      // println!("{:?}",b);
                                      let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                  }
                              }
                          }
                          pub fn brotli_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = Vec::new();
                              {
                                  let mut b = CompressorWriter::new(&mut b, 4096, 4, 22);
                                  b.write_all(billionaire.message.as_bytes()).unwrap();
                                  b.flush().unwrap();
                              }
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn zstd_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut input = Cursor::new(&billionaire.message);
                              let b = encode_all(&mut input, 3).unwrap();
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn param_parser(parts:Vec<&str>,user_parts:Vec<&str>, bhash:&mut HashMap<String,String>){
                              for (b, billion) in parts.iter().zip(user_parts.iter()) {
                                  // println!("{}{}", b, billion);
                                  if b != billion && billion.starts_with(&"<") && billion.ends_with(&">") {
                                      let key = billion.replace("<", "").replace(">", "");
                                      let value = b;
                                      bhash.insert(key.to_string(), value.to_string());
                                  }
                              }
                          }
                      pub fn zen(response:Return,mut stream:&TcpStream,header:Vec<(&str, &str)>,compression:&str,secure_header:&str) {
                              // println!("{}",pathb);
                      // println!("{:?}",compression);
                      // println!("{:?}",&response.encoding);
                      // println!("{}","billionairegreathari");
                      let mut chunks = BufWriter::new(&mut stream);
                      
                      let billionaire = response;
                      // Return { status: 200, message: ["{\"name\":\"billionaireid\"}"], content_type: Some("application/json"), headers: Some([]) }
                      
                      let mut response = format!("HTTP/1.1 {} OK\r\n\
                                  Connection: keep-alive\r\n\
                                  X-Content-Type-Options: nosniff\r\n\
                                  Transfer-Encoding: chunked\r\n",&billionaire.status);
                      
                                            if secure_header.len() != 0 {
                        response.push_str(&format!("{}\r\n",secure_header));
                        // println!("{}",billionaire);
                      }
                      // println!("{:?}",header); 
                      if header.len() >= 1 {
                        for header in header.iter(){
                          // println!("{:?}",header);;
                          response.push_str(&format!("{}: {}\r\n",header.0,header.1));
                        }
                      }                               
                      if !response.contains("Content-Security-Policy: "){
                        response.push_str(&format!("Content-Security-Policy: default-src 'self'\r\n"));
                      }
                      if !response.contains("X-Frame-Options: "){
                        response.push_str(&format!("X-Frame-Options: DENY\r\n"));
                      }
                      if !response.contains("Referrer-Policy: "){
                        response.push_str(&format!("Referrer-Policy: no-referrer\r\n"));
                      }
                      if !response.contains("Keep-Alive: "){
                        response.push_str(&format!("Keep-Alive: timeout=10, max=100\r\n"));
                      }
                      if !response.contains("Cache-Control: "){
                        response.push_str(&format!("Cache-Control: public; max-age: 5; stale-while-revalidate: 10\r\n"));
                      }
                      if let Some(res) = &billionaire.content_type {
                      //   println!("{}",res);
                        response.push_str(&format!("Content-Type: {}; charset=UTF-8\r\n",res));
                      }
                      
                      
                      // println!("{}",response);
                    
                    if compression == "" && billionaire.encoding.is_none() {
                      // println!("{}","billionaire");
                      response.push_str("Content-Encoding: gzip");
                      response.push_str("\r\n\r\n");
                      // println!("{}",response);
                      chunks.write_all(response.as_bytes()).unwrap();
                      // println!("{:?}",billionaire);
                      gzip_compress(&billionaire,&mut chunks);
                    }
                    if let Some(b) = &billionaire.encoding {
                        if b == "gzip" {
                          response.push_str("Content-Encoding: gzip");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          // println!("{:?}",billionaire);
                          gzip_compress(&billionaire,&mut chunks);
  
                        }
                        else if b == "brotli" {
                          response.push_str("Content-Encoding: br");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          brotli_compress(&billionaire,&mut chunks);
                        }
                        else if b == "zstd" {
                          response.push_str("Content-Encoding: zstd");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          zstd_compress(&billionaire,&mut chunks);
  
  
                        }
                    }
                    else {
                      if compression == "gzip" {
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      else if compression == "brotli" {
                        response.push_str("Content-Encoding: br");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        brotli_compress(&billionaire,&mut chunks);
                      }
                      else if compression == "zstd" {
                        response.push_str("Content-Encoding: zstd");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        zstd_compress(&billionaire,&mut chunks);
                      }
                    }
                      chunks.write_all("0\r\n\r\n".as_bytes()).unwrap();
                      chunks.flush().unwrap();     
                      
                    }
              }
  
                      #block
                      // if let Some(function) = 
                      // println!("{:?}",b);
                      
                      // match b {
                      //     Ok(b) => {
                              // println!("{}",b);
                      //     }
                      //     Err(error) => {
                              
                      //         let b = format!("HTTP/1.1 {} NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",error.status,error.message.len(),error.message);
                              // println!("{}",b);
                      //         let _ = stream.write_all(b.as_bytes());
                      //         let _ = stream.flush();
                      //         // return;
                      //     }
                      // }
                      //     let gatekeeper = {#function()};
                      //     println!("{:?}",function);
                      // };
                      let methodb = response.method();
                      let pathb = response.path();
                      let len = pathb.find("?").or_else(|| Some(pathb.len())).unwrap();
                      let real_path = &pathb[..len];
                      // println!("from {}",pathb);
                      // println!("{}",compression);
                      // println!("billionaire {}",#path);
                      // println!("{}",pathb == #path);
          
                      // println!("{}",real_path);
  
                      let parts:Vec<&str> = pathb.split("/").collect();
                      // println!("{:?}",parts);
                      let user_parts:Vec<&str> = #path.split("/").collect();
                      // println!("bbbbbb {}",parts.len() == user_parts.len());
                      // println!("{:?}",user_parts);
          
                      let mut billionaires = HashMap::new();
          
                      if methodb == #method {
                          // println!("{}",methodb);
                          // println!("{}","++++++++++++++++++++++++++");
                          if real_path == #path {
                              // println!("========={}","billionairegreat");
                              // println!("{:?}",response.query());
                              let billionaire = GatesRequest {
                                  body: "",
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query(),
                              };
                              // println!("{:?}",billionaire);
                              // let response = {#fn_name(&billionaire)};
                              let response = {#fn_name(&billionaire)};
                              // println!("{}","peek");
                              if let GateKeeperResponse::Response(response) = response {
                                // println!("{:?}",response);
                                bucket::zen(response,stream,header,compression,secure_header);
                                return;
                              }
                              else if let GateKeeperResponse::Redirect(b,path) = response {
                                let status = match b {
                                  302 => "Temproary Redirect",
                                  301 => "Permanently Moved",
                                  307 => "Temproary Moved",
                                  _ => "OK"
                                };
                              let b = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",b,status,path);
                              let _ = stream.write_all(b.as_bytes()).unwrap();
                              let _ = stream.flush().unwrap();
                              return;
                            }
                                  // chunks.write_all(billionaire.0.as_bytes());
                                  // // chunks.write_all(&compress);
                                  // // println!("{:?}",b);
                          }
                        // }
                        else if #path.contains("<") && #path.contains(">")  && !["/favicon.ico"].contains(&pathb) && parts.len() == user_parts.len() {
                              // #b;
                              // println!("{}","billionaire++++++++++++++++++++++++++");
                              // println!("{}","billionaires");
                              bucket::param_parser(parts,user_parts,&mut billionaires);
                              // println!("======={:?}",billionaires);
                              let billionaire = GatesRequest {
                                  body: "",
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query()
                                  };

                              let response = {#fn_name(&billionaire)};
                              if let GateKeeperResponse::Response(response) = response {
                                bucket::zen(response,stream,header,compression,secure_header);
                                return ;
                              }
                                else if let GateKeeperResponse::Redirect(b,path) = response {
                                    let status = match b {
                                      302 => "Temproary Redirect",
                                      301 => "Permanently Moved",
                                      307 => "Temproary Moved",
                                      _ => "OK"
                                    };
                                  let b = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",b,status,path);
                                  let _ = stream.write_all(b.as_bytes()).unwrap();
                                  let _ = stream.flush().unwrap();
                                  return;
                                }
                        }
                      }
                    }
                  }
              .into()
          }
          "POST" => {
              quote! {
                                    // use nous::TcpStream;
                  // use nous::Response;
                  // use nous::HashMap;
                  // // use nous::Write;                  use nous::HashMap;
                  // use nous::Response;
                  // // use nous::TcpStream;
                  // use nous::GatesRequest;
                  // use std::io::BufWriter;
                  // use nous::flate2::write::GzEncoder;
                  // use nous::flate2::Compression;
                  // use nous::brotli::CompressorWriter;
                  // use std::io::Cursor;
                  // use nous::zstd::encode_all;
                  // use nous::Write;
                  // use nous::GateKeeperResponse;
                  // use nous::TcpStream;
                  // use nous::Response;
                  // use nous::HashMap;
                  // use nous::Write;
                  pub fn #fn_name(mut stream:&TcpStream,response:&Response,compression:&str,secure_header:&str,header:Vec<(&str, &str)>) {
                      mod bucket {
                          use nous::HashMap;
                          use nous::Response;
                          use nous::TcpStream;
                          use nous::GatesRequest;
                          use std::io::BufWriter;
                          use nous::flate2::write::GzEncoder;
                          use nous::flate2::Compression;
                          use nous::brotli::CompressorWriter;
                          use std::io::Cursor;
                          use nous::zstd::encode_all;
                          use nous::Write;
                          // use nous::GateKeeperResponse;
                          // use nous::GateKeeperResponse;
                          use nous::Return;

                          pub fn gzip_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = GzEncoder::new(Vec::new(), Compression::default());
                              b.write_all(&billionaire.message.as_bytes()).unwrap();
                              let b = b.finish();
                              if let Ok(response) = b {
                                  for b in response.chunks(6) {
                                      // hexa of chunk\r\n
                                      // chunk\r\n
                                      let billionaire = format!("{:X}\r\n", b.len());
                                      // println!("{}",billionaire);
                                      let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                      let _ = stream.write_all(b).unwrap();
                                      // println!("{:?}",b);
                                      let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                  }
                              }
                          }
                          pub fn brotli_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = Vec::new();
                              {
                                  let mut b = CompressorWriter::new(&mut b, 4096, 11, 22);
                                  b.write_all(billionaire.message.as_bytes()).unwrap();
                                  b.flush().unwrap();
                              }
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn zstd_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut input = Cursor::new(&billionaire.message);
                              let b = encode_all(&mut input, 3).unwrap();
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn param_parser(parts:Vec<&str>,user_parts:Vec<&str>, bhash:&mut HashMap<String,String>){
                              for (b, billion) in parts.iter().zip(user_parts.iter()) {
                                  // println!("{}{}", b, billion);
                                  if b != billion && billion.starts_with(&"<") && billion.ends_with(&">") {
                                      let key = billion.replace("<", "").replace(">", "");
                                      let value = b;
                                      bhash.insert(key.to_string(), value.to_string());
                                  }
                              }
                          }
                          pub fn zen(response:Return,mut stream:&TcpStream,header:Vec<(&str, &str)>,compression:&str,secure_header:&str) {
                              // println!("{}",pathb);
                      // println!("{}",compression);
                      // println!("{}","billionairegreathari");
                      let mut chunks = BufWriter::new(&mut stream);
                      
                      let billionaire = response;
                      // Return { status: 200, message: ["{\"name\":\"billionaireid\"}"], content_type: Some("application/json"), headers: Some([]) }
                      
                      let mut response = format!("HTTP/1.1 {} OK\r\n\
                                  Connection: keep-alive\r\n\
                                  X-Content-Type-Options: nosniff\r\n\
                                  Transfer-Encoding: chunked\r\n",&billionaire.status);
                      
                                            if secure_header.len() != 0 {
                        response.push_str(&format!("{}\r\n",secure_header));
                        // println!("{}",billionaire);
                      }
                      if secure_header.len() != 0 {
                        response.push_str(&format!("{}\r\n",secure_header));
                      }
                      // println!("{:?}",header); 
                      if header.len() >= 1 {
                        for header in header.iter(){
                          // println!("{:?}",header);;
                          response.push_str(&format!("{}: {}\r\n",header.0,header.1));
                        }
                      }                               
                      if !response.contains("Content-Security-Policy: "){
                        response.push_str(&format!("Content-Security-Policy: default-src 'self'\r\n"));
                      }
                      if !response.contains("X-Frame-Options: "){
                        response.push_str(&format!("X-Frame-Options: DENY\r\n"));
                      }
                      if !response.contains("Referrer-Policy: "){
                        response.push_str(&format!("Referrer-Policy: no-referrer\r\n"));
                      }
                      if !response.contains("Keep-Alive: "){
                        response.push_str(&format!("Keep-Alive: timeout=10, max=100\r\n"));
                      }
                      if !response.contains("Cache-Control: "){
                        response.push_str(&format!("Cache-Control: public; max-age: 5; stale-while-revalidate: 10\r\n"));
                      }
                      if let Some(res) = &billionaire.content_type {
                      //   println!("{}",res);
                        response.push_str(&format!("Content-Type: {}; charset=UTF-8\r\n",res));
                      }
                      
                      
                      // println!("{}",response);
                      if compression == "" && billionaire.encoding.is_none(){
                        // println!("{}","billionaire");
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        // println!("{:?}",billionaire);
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      if let Some(b) = &billionaire.encoding {
                        if b == "gzip" {
                          response.push_str("Content-Encoding: gzip");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          // println!("{:?}",billionaire);
                          gzip_compress(&billionaire,&mut chunks);
  
                        }
                        else if b == "brotli" {
                          response.push_str("Content-Encoding: br");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          brotli_compress(&billionaire,&mut chunks);
                        }
                        else if compression == "zstd" {
                          response.push_str("Content-Encoding: zstd");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          zstd_compress(&billionaire,&mut chunks);
  
  
                        }
                      }
                      else {
                      if compression == "gzip" {
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      else if compression == "brotli" {
                        response.push_str("Content-Encoding: br");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        brotli_compress(&billionaire,&mut chunks);
  
  
                      }
                      else if compression == "zstd" {
                        response.push_str("Content-Encoding: zstd");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        zstd_compress(&billionaire,&mut chunks);
                      }
                      }
                      chunks.write_all("0\r\n\r\n".as_bytes()).unwrap();
                      chunks.flush().unwrap();     
                      
                      }
                      }
  
                      #block

                      let methodb = response.method();
                      let pathb = response.path();
                      let len = pathb.find("?").or_else(|| Some(pathb.len())).unwrap();
                      let real_path = &pathb[..len];
                      // println!("from {}",pathb);
                      // println!("{}",compression);
                      // println!("billionaire {}",#path);
                      // println!("{}",pathb == #path);
          
                      let parts:Vec<&str> = pathb.split("/").collect();
                      // println!("{:?}",parts);
                      let user_parts:Vec<&str> = #path.split("/").collect();
                      // println!("bbbbbb {}",parts.len() == user_parts.len());
                      // println!("{:?}",user_parts);
          
                      let mut billionaires = HashMap::new();
          
                      if methodb == #method {
                          // println!("{}",methodb);
                          if real_path == #path {
                              // println!("{}","billionairegreat");
                              // println!("{:?}",response);
                              let billionaire = GatesRequest {
                                  body: response.content(),
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query(),
                              };
                              let response = {#fn_name(&billionaire)};
                                // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                              if let GateKeeperResponse::Response(response) = response {
                                bucket::zen(response,stream,header,compression,secure_header);
                              }   
                              else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                let status = match code {
                                  302 => "Temproary Redirect",
                                  301 => "Permanently Moved",
                                  307 => "Temproary Moved",
                                  _ => "OK"
                                };
                                let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                let _ = stream.write_all(response.as_bytes()).unwrap();
                                let _ = stream.flush().unwrap();
                                // println!("{}",response);
                                return;
                              }
                                  // chunks.write_all(billionaire.0.as_bytes());
                                  // // chunks.write_all(&compress);
                                  // // println!("{:?}",b);
                          }
                          else if #path.contains("<") && #path.contains(">")  && !["/favicon.ico"].contains(&pathb) && parts.len() == user_parts.len() {
                              // #b;
                              // println!("{}","billionaires");
                              bucket::param_parser(parts,user_parts,&mut billionaires);
                              // println!("{:?}",billionaires);
                              let billionaire = GatesRequest {
                                  body: response.content(),
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query()
                                  };
                                // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                              let response = {#fn_name(&billionaire)};
                              if let GateKeeperResponse::Response(response) = response {
                                bucket::zen(response,stream,header,compression,secure_header);
                              }   
                              else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                let status = match code {
                                  302 => "Temproary Redirect",
                                  301 => "Permanently Moved",
                                  307 => "Temproary Moved",
                                  _ => "OK"
                                };
                                let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                let _ = stream.write_all(response.as_bytes()).unwrap();
                                let _ = stream.flush().unwrap();
                                // println!("{}",response);
                                return;
                              }
                          }
                      }
                  }
              }
              .into()
          }
          "PUT" => {
              quote! {
                  //                   use nous::TcpStream;
                  // use nous::Response;
                  // use nous::HashMap;
                  // // use nous::Write;                  use nous::HashMap;
                  // use nous::Response;
                  // // use nous::TcpStream;
                  // use nous::GatesRequest;
                  // use std::io::BufWriter;
                  // use nous::flate2::write::GzEncoder;
                  // use nous::flate2::Compression;
                  // use nous::brotli::CompressorWriter;
                  // use std::io::Cursor;
                  // use nous::zstd::encode_all;
                  // use nous::Write;
                  // use nous::GateKeeperResponse;
                  // use nous::TcpStream;
                  // use nous::Response;
                  // use nous::HashMap;
                  // use nous::Write;
                  pub fn #fn_name(mut stream:&TcpStream,response:&Response,compression:&str,secure_header:&str,header:Vec<(&str, &str)>) {
                      mod bucket {
                          use nous::HashMap;
                          use nous::Response;
                          use nous::TcpStream;
                          use nous::GatesRequest;
                          use std::io::BufWriter;
                          use nous::flate2::write::GzEncoder;
                          use nous::flate2::Compression;
                          use nous::brotli::CompressorWriter;
                          use std::io::Cursor;
                          use nous::zstd::encode_all;
                          use nous::Write;
                          // use nous::GateKeeperResponse;
                          // use crate::#function;
                          // use crate::#fn_name;
                          use nous::Return;
                          pub fn gzip_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = GzEncoder::new(Vec::new(), Compression::default());
                              b.write_all(&billionaire.message.as_bytes()).unwrap();
                              let b = b.finish();
                              if let Ok(response) = b {
                                  for b in response.chunks(6) {
                                      // hexa of chunk\r\n
                                      // chunk\r\n
                                      let billionaire = format!("{:X}\r\n", b.len());
                                      // println!("{}",billionaire);
                                      let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                      let _ = stream.write_all(b).unwrap();
                                      // println!("{:?}",b);
                                      let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                  }
                              }
                          }
                          pub fn brotli_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = Vec::new();
                              {
                                  let mut b = CompressorWriter::new(&mut b, 4096, 11, 22);
                                  b.write_all(billionaire.message.as_bytes()).unwrap();
                                  b.flush().unwrap();
                              }
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn zstd_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut input = Cursor::new(&billionaire.message);
                              let b = encode_all(&mut input, 3).unwrap();
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn param_parser(parts:Vec<&str>,user_parts:Vec<&str>, bhash:&mut HashMap<String,String>){
                              for (b, billion) in parts.iter().zip(user_parts.iter()) {
                                  // println!("{}{}", b, billion);
                                  if b != billion && billion.starts_with(&"<") && billion.ends_with(&">") {
                                      let key = billion.replace("<", "").replace(">", "");
                                      let value = b;
                                      bhash.insert(key.to_string(), value.to_string());
                                  }
                              }
                          }
                          pub fn zen(response:Return,mut stream:&TcpStream,header:Vec<(&str, &str)>,compression:&str,secure_header:&str) {
                              // println!("{}",pathb);
                      // println!("{}",compression);
                      // println!("{}","billionairegreathari");
                      let mut chunks = BufWriter::new(&mut stream);
                      
                      let billionaire = response;
                      // Return { status: 200, message: ["{\"name\":\"billionaireid\"}"], content_type: Some("application/json"), headers: Some([]) }
                      
                      let mut response = format!("HTTP/1.1 {} OK\r\n\
                                  Connection: keep-alive\r\n\
                                  X-Content-Type-Options: nosniff\r\n\
                                  Transfer-Encoding: chunked\r\n",&billionaire.status);
                      
                      if secure_header.len() != 0 {
                        response.push_str(&format!("{}\r\n",secure_header));
                      }
                      // println!("{:?}",header); 
                      if header.len() >= 1 {
                        for header in header.iter(){
                          // println!("{:?}",header);;
                          response.push_str(&format!("{}: {}\r\n",header.0,header.1));
                        }
                      }                               
                      if !response.contains("Content-Security-Policy: "){
                        response.push_str(&format!("Content-Security-Policy: default-src 'self'\r\n"));
                      }
                      if !response.contains("X-Frame-Options: "){
                        response.push_str(&format!("X-Frame-Options: DENY\r\n"));
                      }
                      if !response.contains("Referrer-Policy: "){
                        response.push_str(&format!("Referrer-Policy: no-referrer\r\n"));
                      }
                      if !response.contains("Keep-Alive: "){
                        response.push_str(&format!("Keep-Alive: timeout=10, max=100\r\n"));
                      }
                      if !response.contains("Cache-Control: "){
                        response.push_str(&format!("Cache-Control: public; max-age: 5; stale-while-revalidate: 10\r\n"));
                      }
                      if let Some(res) = &billionaire.content_type {
                      //   println!("{}",res);
                        response.push_str(&format!("Content-Type: {}; charset=UTF-8\r\n",res));
                      }
                      
                      
                      // println!("{}",response);
                      if compression == "" && billionaire.encoding.is_none(){
                        // println!("{}","billionaire");
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        // println!("{:?}",billionaire);
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      if let Some(b) = &billionaire.encoding {
                        if b == "gzip" {
                          response.push_str("Content-Encoding: gzip");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          // println!("{:?}",billionaire);
                          gzip_compress(&billionaire,&mut chunks);
  
                        }
                        else if b == "brotli" {
                          response.push_str("Content-Encoding: br");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          brotli_compress(&billionaire,&mut chunks);
                        }
                        else if compression == "zstd" {
                          response.push_str("Content-Encoding: zstd");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          zstd_compress(&billionaire,&mut chunks);
  
  
                        }
                      }
                      else {
                      if compression == "gzip" {
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      else if compression == "brotli" {
                        response.push_str("Content-Encoding: br");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        brotli_compress(&billionaire,&mut chunks);
  
  
                      }
                      else if compression == "zstd" {
                        response.push_str("Content-Encoding: zstd");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        zstd_compress(&billionaire,&mut chunks);
                      }
                      }
                      chunks.write_all("0\r\n\r\n".as_bytes()).unwrap();
                      chunks.flush().unwrap();     
                      
                      }
                      }
  
                      #block

                      let methodb = response.method();
                      let pathb = response.path();
                      let len = pathb.find("?").or_else(|| Some(pathb.len())).unwrap();
                      let real_path = &pathb[..len];
                      // println!("from {}",pathb);
                      // println!("{}",compression);
                      // println!("billionaire {}",#path);
                      // println!("{}",pathb == #path);
          
                      let parts:Vec<&str> = pathb.split("/").collect();
                      // println!("{:?}",parts);
                      let user_parts:Vec<&str> = #path.split("/").collect();
                      // println!("bbbbbb {}",parts.len() == user_parts.len());
                      // println!("{:?}",user_parts);
          
                      let mut billionaires = HashMap::new();
          
                      if methodb == #method {
                          // println!("{}",methodb);
                          if real_path == #path {
                              // println!("{}","billionairegreat");
                              // println!("{:?}",response);
                              let billionaire = GatesRequest {
                                  body: response.content(),
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query(),
                              };
                              let response = {#fn_name(&billionaire)};
                              // println!("{}","billionaires");
                                // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                                if let GateKeeperResponse::Response(response) = response {
                                  bucket::zen(response,stream,header,compression,secure_header);
                                }   
                                else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                  let status = match code {
                                    302 => "Temproary Redirect",
                                    301 => "Permanently Moved",
                                    307 => "Temproary Moved",
                                    _ => "OK"
                                  };
                                  let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                  let _ = stream.write_all(response.as_bytes()).unwrap();
                                  let _ = stream.flush().unwrap();
                                  // println!("{}",response);
                                  return;
                                }
                                  // chunks.write_all(billionaire.0.as_bytes());
                                  // // chunks.write_all(&compress);
                                  // // println!("{:?}",b);
                          }
                          else if #path.contains("<") && #path.contains(">")  && !["/favicon.ico"].contains(&pathb) && parts.len() == user_parts.len() {
                              // #b;
                              // println!("{}","billionaires");
                              bucket::param_parser(parts,user_parts,&mut billionaires);
                              // println!("{:?}",billionaires);
                              let billionaire = GatesRequest {
                                  body: response.content(),
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query()
                                  };
                                // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);

                              let response = {#fn_name(&billionaire)};
                              if let GateKeeperResponse::Response(response) = response {
                                bucket::zen(response,stream,header,compression,secure_header);
                              }   
                              else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                let status = match code {
                                  302 => "Temproary Redirect",
                                  301 => "Permanently Moved",
                                  307 => "Temproary Moved",
                                  _ => "OK"
                                };
                                let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                let _ = stream.write_all(response.as_bytes()).unwrap();
                                let _ = stream.flush().unwrap();
                                // println!("{}",response);
                                return;
                              }
                          }
                      }
                  }
              }
              .into()
          }
          "DELETE" => {
              quote! {
                  //                   use nous::TcpStream;
                  // use nous::Response;
                  // use nous::HashMap;
                  // // use nous::Write;                  use nous::HashMap;
                  // use nous::Response;
                  // // use nous::TcpStream;
                  // use nous::GatesRequest;
                  // use std::io::BufWriter;
                  // use nous::flate2::write::GzEncoder;
                  // use nous::flate2::Compression;
                  // use nous::brotli::CompressorWriter;
                  // use std::io::Cursor;
                  // use nous::zstd::encode_all;
                  // use nous::Write;
                  // use nous::GateKeeperResponse;
                  // use nous::TcpStream;
                  // use nous::Response;
                  // use nous::HashMap;
                  // use nous::Write;
                  pub fn #fn_name(mut stream:&TcpStream,response:&Response,compression:&str,secure_header:&str,header:Vec<(&str, &str)>) {
                      mod bucket {
                          use nous::HashMap;
                          use nous::Response;
                          use nous::TcpStream;
                          use nous::GatesRequest;
                          use std::io::BufWriter;
                          use nous::flate2::write::GzEncoder;
                          use nous::flate2::Compression;
                          use nous::brotli::CompressorWriter;
                          use std::io::Cursor;
                          use nous::zstd::encode_all;
                          use nous::Write;
                          // use nous::GateKeeperResponse;
                          // use crate::#function;
                          // use crate::#fn_name;
                          use nous::Return;
                          pub fn gzip_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = GzEncoder::new(Vec::new(), Compression::default());
                              b.write_all(&billionaire.message.as_bytes()).unwrap();
                              let b = b.finish();
                              if let Ok(response) = b {
                                  for b in response.chunks(6) {
                                      // hexa of chunk\r\n
                                      // chunk\r\n
                                      let billionaire = format!("{:X}\r\n", b.len());
                                      // println!("{}",billionaire);
                                      let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                      let _ = stream.write_all(b).unwrap();
                                      // println!("{:?}",b);
                                      let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                  }
                              }
                          }
                          pub fn brotli_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = Vec::new();
                              {
                                  let mut b = CompressorWriter::new(&mut b, 4096, 11, 22);
                                  b.write_all(billionaire.message.as_bytes()).unwrap();
                                  b.flush().unwrap();
                              }
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn zstd_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut input = Cursor::new(&billionaire.message);
                              let b = encode_all(&mut input, 3).unwrap();
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn param_parser(parts:Vec<&str>,user_parts:Vec<&str>, bhash:&mut HashMap<String,String>){
                              for (b, billion) in parts.iter().zip(user_parts.iter()) {
                                  // println!("{}{}", b, billion);
                                  if b != billion && billion.starts_with(&"<") && billion.ends_with(&">") {
                                      let key = billion.replace("<", "").replace(">", "");
                                      let value = b;
                                      bhash.insert(key.to_string(), value.to_string());
                                  }
                              }
                          }
                          pub fn zen(response:Return,mut stream:&TcpStream,header:Vec<(&str, &str)>,compression:&str,secure_header:&str) {
                              // println!("{}",pathb);
                      // println!("{}",compression);
                      // println!("{}","billionairegreathari");
                      let mut chunks = BufWriter::new(&mut stream);
                      
                      let billionaire = response;
                      // Return { status: 200, message: ["{\"name\":\"billionaireid\"}"], content_type: Some("application/json"), headers: Some([]) }
                      
                      let mut response = format!("HTTP/1.1 {} OK\r\n\
                                  Connection: keep-alive\r\n\
                                  X-Content-Type-Options: nosniff\r\n\
                                  Transfer-Encoding: chunked\r\n",&billionaire.status);
                      
                      if secure_header.len() != 0 {
                        response.push_str(&format!("{}\r\n",secure_header));
                      }
                      // println!("{:?}",header); 
                      if header.len() >= 1 {
                        for header in header.iter(){
                          // println!("{:?}",header);;
                          response.push_str(&format!("{}: {}\r\n",header.0,header.1));
                        }
                      }                               
                      if !response.contains("Content-Security-Policy: "){
                        response.push_str(&format!("Content-Security-Policy: default-src 'self'\r\n"));
                      }
                      if !response.contains("X-Frame-Options: "){
                        response.push_str(&format!("X-Frame-Options: DENY\r\n"));
                      }
                      if !response.contains("Referrer-Policy: "){
                        response.push_str(&format!("Referrer-Policy: no-referrer\r\n"));
                      }
                      if !response.contains("Keep-Alive: "){
                        response.push_str(&format!("Keep-Alive: timeout=10, max=100\r\n"));
                      }
                      if !response.contains("Cache-Control: "){
                        response.push_str(&format!("Cache-Control: public; max-age: 5; stale-while-revalidate: 10\r\n"));
                      }
                      if let Some(res) = &billionaire.content_type {
                      //   println!("{}",res);
                        response.push_str(&format!("Content-Type: {}; charset=UTF-8\r\n",res));
                      }
                      
                      
                      // println!("{}",response);
                      if compression == "" && billionaire.encoding.is_none(){
                        // println!("{}","billionaire");
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        // println!("{:?}",billionaire);
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      if let Some(b) = &billionaire.encoding {
                        if b == "gzip" {
                          response.push_str("Content-Encoding: gzip");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          // println!("{:?}",billionaire);
                          gzip_compress(&billionaire,&mut chunks);
  
                        }
                        else if b == "brotli" {
                          response.push_str("Content-Encoding: br");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          brotli_compress(&billionaire,&mut chunks);
                        }
                        else if compression == "zstd" {
                          response.push_str("Content-Encoding: zstd");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          zstd_compress(&billionaire,&mut chunks);
  
  
                        }
                      }
                      else {
                      if compression == "gzip" {
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      else if compression == "brotli" {
                        response.push_str("Content-Encoding: br");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        brotli_compress(&billionaire,&mut chunks);
  
  
                      }
                      else if compression == "zstd" {
                        response.push_str("Content-Encoding: zstd");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        zstd_compress(&billionaire,&mut chunks);
                      }
                      }
                      chunks.write_all("0\r\n\r\n".as_bytes()).unwrap();
                      chunks.flush().unwrap();     
                      
                      }
                      }
  
                      #block

                      let methodb = response.method();
                      let pathb = response.path();
                      let len = pathb.find("?").or_else(|| Some(pathb.len())).unwrap();
                      let real_path = &pathb[..len];
                      // println!("from {}",pathb);
                      // println!("{}",compression);
                      // println!("billionaire {}",#path);
                      // println!("{}",pathb == #path);
          
                      let parts:Vec<&str> = pathb.split("/").collect();
                      // println!("{:?}",parts);
                      let user_parts:Vec<&str> = #path.split("/").collect();
                      // println!("bbbbbb {}",parts.len() == user_parts.len());
                      // println!("{:?}",user_parts);
          
                      let mut billionaires = HashMap::new();
          
                      if methodb == #method {
                          // println!("{}",methodb);
                          if real_path == #path {
                              // println!("{}","billionairegreat");
                              // println!("{:?}",response);
                              let billionaire = GatesRequest {
                                  body: response.content(),
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query(),
                              };
                              let response = {#fn_name(&billionaire)};
                                // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                                if let GateKeeperResponse::Response(response) = response {
                                  bucket::zen(response,stream,header,compression,secure_header);
                                }   
                                else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                  let status = match code {
                                    302 => "Temproary Redirect",
                                    301 => "Permanently Moved",
                                    307 => "Temproary Moved",
                                    _ => "OK"
                                  };
                                  let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                  let _ = stream.write_all(response.as_bytes()).unwrap();
                                  let _ = stream.flush().unwrap();
                                  // println!("{}",response);
                                  return;
                                }
                                  // chunks.write_all(billionaire.0.as_bytes());
                                  // // chunks.write_all(&compress);
                                  // // println!("{:?}",b);
                          }
                          else if #path.contains("<") && #path.contains(">")  && !["/favicon.ico"].contains(&pathb) && parts.len() == user_parts.len() {
                              // #b;
                              // println!("{}","billionaires");
                              bucket::param_parser(parts,user_parts,&mut billionaires);
                              // println!("{:?}",billionaires);
                              let billionaire = GatesRequest {
                                  body: response.content(),
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query()
                                  };
                                // println!("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb {:?}",gatekeeper);
                              let response = {#fn_name(&billionaire)};
                              if let GateKeeperResponse::Response(response) = response {
                                bucket::zen(response,stream,header,compression,secure_header);
                              }   
                              else if let GateKeeperResponse::Redirect(code,redirect) = response {
                                let status = match code {
                                  302 => "Temproary Redirect",
                                  301 => "Permanently Moved",
                                  307 => "Temproary Moved",
                                  _ => "OK"
                                };
                                let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                                let _ = stream.write_all(response.as_bytes()).unwrap();
                                let _ = stream.flush().unwrap();
                                // println!("{}",response);
                                return;
                              }
                          }
                      }
                  }
              }
              .into()
          }
          "text/event-stream" => {
            // println!("{}","billionairegeathari");
            quote! {
              // use nous::HashMap;
              // use nous::Response;
              // use nous::TcpStream;
              // use nous::GatesRequest;
              // use std::io::BufWriter;
              // use nous::flate2::write::GzEncoder;
              // use nous::flate2::Compression;
              // use nous::brotli::CompressorWriter;
              // use std::io::Cursor;
              // use nous::zstd::encode_all;
              // use nous::Write;
              // use nous::GateKeeperResponse;
              pub fn #fn_name(mut stream:&TcpStream,response:&Response,compression:&str,secure_header:&str,header:Vec<(&str, &str)>) {
                // println!("{}","billionaires");
                  mod bucket {
                      use nous::HashMap;
                      use nous::Response;
                      use nous::TcpStream;
                      use nous::GatesRequest;
                      use std::io::BufWriter;
                      use nous::Write;
                      // use nous::GateKeeperResponse;
                      // use crate::#function;
                      // use crate::#fn_name;
                      use nous::Return;
                      pub fn param_parser(parts:Vec<&str>,user_parts:Vec<&str>, bhash:&mut HashMap<String,String>){
                          for (b, billion) in parts.iter().zip(user_parts.iter()) {
                              // println!("{}{}", b, billion);
                              if b != billion && billion.starts_with(&"<") && billion.ends_with(&">") {
                                  let key = billion.replace("<", "").replace(">", "");
                                  let value = b;
                                  bhash.insert(key.to_string(), value.to_string());
                              }
                          }
                      }
                      pub fn zen(response:Return,mut stream:&TcpStream,header:Vec<(&str, &str)>,compression:&str,secure_header:&str) {
                          // println!("{}",pathb);
                          // println!("{}",compression);
                          // println!("{}","billionairegreathari");
                          let mut chunks = BufWriter::new(&mut stream);
                          
                          let billionaire = response;
                          // Return { status: 200, message: ["{\"name\":\"billionaireid\"}"], content_type: Some("application/json"), headers: Some([]) }
                          
                          let mut response = format!("HTTP/1.1 {} OK\r\n\
                                      Connection: keep-alive\r\n\
                                      X-Content-Type-Options: nosniff\r\n\
                                      Content-Type: text/event-stream\r\n",&billionaire.status);
                                      // Transfer-Encoding: chunked\r\n
                          
                          if secure_header.len() != 0 {
                              response.push_str(&format!("{}\r\n",secure_header));
                          }
                          // println!("{:?}",header); 
                          if header.len() >= 1 {
                            for header in header.iter(){
                              // println!("{:?}",header);;
                                response.push_str(&format!("{}: {}\r\n",header.0,header.1));
                            }
                          }                               
                          if !response.contains("Content-Security-Policy: "){
                            response.push_str(&format!("Content-Security-Policy: default-src 'self'\r\n"));
                          }
                          if !response.contains("X-Frame-Options: "){
                            response.push_str(&format!("X-Frame-Options: DENY\r\n"));
                          }
                          if !response.contains("Referrer-Policy: "){
                            response.push_str(&format!("Referrer-Policy: no-referrer\r\n"));
                          }
                          if !response.contains("Keep-Alive: "){
                            response.push_str(&format!("Keep-Alive: timeout=10, max=100\r\n"));
                          }
                          if !response.contains("Cache-Control: "){
                            response.push_str(&format!("Cache-Control: no-cache; no-store; must-revalidate;\r\n"));
                          }
                          // if let Some(res) = &billionaire.content_type {
                          // //   println!("{}",res);
                          //   response.push_str(&format!("Content-Type: {}; charset=UTF-8\r\n",res));
                          // }
                          
                          
                          // println!("{}",response);
                          
          
                          let b: Vec<&str> = billionaire.message.split_whitespace().collect();
                          // println!("{:?}", b);
                          response.push_str(&format!("\r\n"));
                          // println!("{:?}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                      
                          for b in b {
                              let billionaire = format!("data: {}\n\n", b);
                              // println!("{:?}",billionaire);
                              chunks.write_all(&billionaire.as_bytes()).unwrap();
                              chunks.flush().unwrap();
                          }
                        }
                    }
                    // println!("{}","billionairebillionaires");
                  #block

                  let methodb = response.method();
                  let pathb = response.path();
                  let len = pathb.find("?").or_else(|| Some(pathb.len())).unwrap();
                  let real_path = &pathb[..len];
                  // println!("from {}",pathb);
                  // println!("{}",compression);
                  // println!("billionaire {}",#path);
                  // println!("{}",pathb == #path);
      
                  // println!("{}",real_path);
  
                  let parts:Vec<&str> = pathb.split("/").collect();
                  // println!("{:?}",parts);
                  let user_parts:Vec<&str> = #path.split("/").collect();
                  // println!("bbbbbb {}",parts.len() == user_parts.len());
                  // println!("{:?}",user_parts);
      
                  let mut billionaires = HashMap::new();
      
                  let headerb = response.header();
                  // println!("{:?}",header);
                  let headerb = headerb.get("Accept").unwrap().trim();
                  // println!("{}",headerb);
                  // println!("{} {}",real_path,#path);
                  if "GET" == methodb && "text/event-stream" == headerb {
                      // println!("==============================={}",methodb);
                      // println!("{}",b);
                      if real_path == #path {
                          // println!("{}","billionairegreat");
                          // println!("{:?}",response.query());
                          // let billionaire = GatesRequest {
                          //     body: "",
                          //     cookies: &response.cookie(),
                          //     headers: response.header(),
                          //     path: response.path(),
                          //     method: response.method(),
                          //     param: billionaires,
                          //     query: response.query(),
                          // };
                          // println!("{:?}",billionaire);
                          let response = {#fn_name()};

                          // println!("{}","peek");
                          if let GateKeeperResponse::Response(response) = response {
                            // println!("{}","from sse billionaire");
                            bucket::zen(response,stream,header,compression,secure_header);
                          }   
                          else if let GateKeeperResponse::Redirect(code,redirect) = response {
                            let status = match code {
                              302 => "Temproary Redirect",
                              301 => "Permanently Moved",
                              307 => "Temproary Moved",
                              _ => "OK"
                            };
                            let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                            let _ = stream.write_all(response.as_bytes()).unwrap();
                            let _ = stream.flush().unwrap();
                            // println!("{}",response);
                            return;
                          }
                              // chunks.write_all(billionaire.0.as_bytes());
                              // // chunks.write_all(&compress);
                              // // println!("{:?}",b);
                      }
                      else if #path.contains("<") && #path.contains(">")  && !["/favicon.ico"].contains(&pathb) && parts.len() == user_parts.len() {
                          // #b;
                          // println!("{}","billionaires");
                          bucket::param_parser(parts,user_parts,&mut billionaires);
                          // println!("{:?}",billionaires);
                          let billionaire = GatesRequest {
                            body: "",
                            cookies: &response.cookie(),
                            headers: response.header(),
                            path: response.path(),
                            method: response.method(),
                            param: billionaires,
                            query: response.query()
                          };
                              // handle this b
                          // let b = {#function(&billionaire)} ;
                          let response = {#fn_name()};
                          if let GateKeeperResponse::Response(response) = response {
                            bucket::zen(response,stream,header,compression,secure_header);
                          }   
                          else if let GateKeeperResponse::Redirect(code,redirect) = response {
                            let status = match code {
                              302 => "Temproary Redirect",
                              301 => "Permanently Moved",
                              307 => "Temproary Moved",
                              _ => "OK"
                            };
                            let response = format!("HTTP/1.1 {} {}\r\nLocation: {}\r\n\r\n",code,status,redirect);
                            let _ = stream.write_all(response.as_bytes()).unwrap();
                            let _ = stream.flush().unwrap();
                            // println!("{}",response);
                            return;
                          }
                      }
                  }
              }
          }
          .into()
          }
          _ => {
            // println!("{}","billionairegreat");
              quote! {
                  //                   use nous::TcpStream;
                  // use nous::Response;
                  // use nous::HashMap;
                  // // use nous::Write;                  use nous::HashMap;
                  // use nous::Response;
                  // // use nous::TcpStream;
                  // use nous::GatesRequest;
                  // use std::io::BufWriter;
                  // use nous::flate2::write::GzEncoder;
                  // use nous::flate2::Compression;
                  // use nous::brotli::CompressorWriter;
                  // use std::io::Cursor;
                  // use nous::zstd::encode_all;
                  // use nous::Write;
                  // use nous::GateKeeperResponse;
                  // use nous::TcpStream;
                  // use nous::Response;
                  // use nous::HashMap;
                  // use nous::Write;
                  pub fn #fn_name(mut stream:&TcpStream,response:&Response,compression:&str,secure_header:&str,header:Vec<(&str, &str)>) {
                      mod bucket {
                          use nous::HashMap;
                          use nous::Response;
                          use nous::TcpStream;
                          use nous::GatesRequest;
                          use std::io::BufWriter;
                          use nous::flate2::write::GzEncoder;
                          use nous::flate2::Compression;
                          use nous::brotli::CompressorWriter;
                          use std::io::Cursor;
                          use nous::zstd::encode_all;
                          use nous::Write;
                          // use nous::GateKeeperResponse;
                          // use crate::#function;
                          // use crate::#fn_name;
                          use nous::Return;
                          pub fn gzip_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = GzEncoder::new(Vec::new(), Compression::default());
                              b.write_all(&billionaire.message.as_bytes()).unwrap();
                              let b = b.finish();
                              if let Ok(response) = b {
                                  for b in response.chunks(6) {
                                      // hexa of chunk\r\n
                                      // chunk\r\n
                                      let billionaire = format!("{:X}\r\n", b.len());
                                      // println!("{}",billionaire);
                                      let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                      let _ = stream.write_all(b).unwrap();
                                      // println!("{:?}",b);
                                      let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                  }
                              }
                          }
                          pub fn brotli_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut b = Vec::new();
                              {
                                  let mut b = CompressorWriter::new(&mut b, 4096, 11, 22);
                                  b.write_all(billionaire.message.as_bytes()).unwrap();
                                  b.flush();
                              }
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn zstd_compress(billionaire:&Return,stream:&mut BufWriter<&mut &TcpStream>) {
                              let mut input = Cursor::new(&billionaire.message);
                              let b = encode_all(&mut input, 3).unwrap();
                                for b in b.chunks(6) {
                                  // hexa of chunk\r\n
                                  // chunk\r\n
                                  let billionaire = format!("{:X}\r\n", b.len());
                                  // println!("{}",billionaire);
                                  let _ = stream.write_all(billionaire.as_bytes()).unwrap();
                                  let _ = stream.write_all(b).unwrap();
                                  // println!("{:?}",b);
                                  let _ = stream.write_all("\r\n".as_bytes()).unwrap();
                                }
                          }
                          pub fn param_parser(parts:Vec<&str>,user_parts:Vec<&str>, bhash:&mut HashMap<String,String>){
                              for (b, billion) in parts.iter().zip(user_parts.iter()) {
                                  // println!("{}{}", b, billion);
                                  if b != billion && billion.starts_with(&"<") && billion.ends_with(&">") {
                                      let key = billion.replace("<", "").replace(">", "");
                                      let value = b;
                                      bhash.insert(key.to_string(), value.to_string());
                                  }
                              }
                          }
                          pub fn zen(response:Return,mut stream:&TcpStream,header:Vec<(&str, &str)>,compression:&str,secure_header:&str) {
                              // println!("{}",pathb);
                      // println!("{}",compression);
                      // println!("{}","billionairegreathari");
                      let mut chunks = BufWriter::new(&mut stream);
                      
                      let billionaire = response;
                      // Return { status: 200, message: ["{\"name\":\"billionaireid\"}"], content_type: Some("application/json"), headers: Some([]) }
                      
                      let mut response = format!("HTTP/1.1 {} OK\r\n\
                                  Connection: keep-alive\r\n\
                                  X-Content-Type-Options: nosniff\r\n\
                                  Transfer-Encoding: chunked\r\n",&billionaire.status);
                      
                      if secure_header.len() != 0 {
                        response.push_str(&format!("{}\r\n",secure_header));
                      }
                      // println!("{:?}",header); 
                      if header.len() >= 1 {
                        for header in header.iter(){
                          // println!("{:?}",header);;
                          response.push_str(&format!("{}: {}\r\n",header.0,header.1));
                        }
                      }                               
                      if !response.contains("Content-Security-Policy: "){
                        response.push_str(&format!("Content-Security-Policy: default-src 'self'\r\n"));
                      }
                      if !response.contains("X-Frame-Options: "){
                        response.push_str(&format!("X-Frame-Options: DENY\r\n"));
                      }
                      if !response.contains("Referrer-Policy: "){
                        response.push_str(&format!("Referrer-Policy: no-referrer\r\n"));
                      }
                      if !response.contains("Keep-Alive: "){
                        response.push_str(&format!("Keep-Alive: timeout=10, max=100\r\n"));
                      }
                      if !response.contains("Cache-Control: "){
                        response.push_str(&format!("Cache-Control: public; max-age: 5; stale-while-revalidate: 10\r\n"));
                      }
                      if let Some(res) = &billionaire.content_type {
                      //   println!("{}",res);
                        response.push_str(&format!("Content-Type: {}; charset=UTF-8\r\n",res));
                      }
                      
                      
                      // println!("{}",response);
                      
                      if let Some(b) = &billionaire.encoding {
                        if b == "gzip" {
                          response.push_str("Content-Encoding: gzip");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          // println!("{:?}",billionaire);
                          gzip_compress(&billionaire,&mut chunks);
  
                        }
                        else if b == "brotli" {
                          response.push_str("Content-Encoding: br");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          brotli_compress(&billionaire,&mut chunks);
                        }
                        else if compression == "zstd" {
                          response.push_str("Content-Encoding: zstd");
                          response.push_str("\r\n\r\n");
                          // println!("{}",response);
                          chunks.write_all(response.as_bytes()).unwrap();
                          zstd_compress(&billionaire,&mut chunks);
  
  
                        }
                      }
                      else {
                      if compression == "gzip" {
                        response.push_str("Content-Encoding: gzip");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        gzip_compress(&billionaire,&mut chunks);
                      }
                      else if compression == "brotli" {
                        response.push_str("Content-Encoding: br");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        brotli_compress(&billionaire,&mut chunks);
  
  
                      }
                      else if compression == "zstd" {
                        response.push_str("Content-Encoding: zstd");
                        response.push_str("\r\n\r\n");
                        // println!("{}",response);
                        chunks.write_all(response.as_bytes()).unwrap();
                        zstd_compress(&billionaire,&mut chunks);
                      }
                      }
                      chunks.write_all("0\r\n\r\n".as_bytes()).unwrap();
                      chunks.flush().unwrap();     
                      
                      }
                      }
  
                      #block

                      let methodb = response.method();
                      let pathb = response.path();
                      let len = pathb.find("?").or_else(|| Some(pathb.len())).unwrap();
                      let real_path = &pathb[..len];
                      // println!("from {}",pathb);
                      // println!("{}",compression);
                      // println!("billionaire {}",#path);
                      // println!("{}",pathb == #path);
          
                      let parts:Vec<&str> = pathb.split("/").collect();
                      // println!("{:?}",parts);
                      let user_parts:Vec<&str> = #path.split("/").collect();
                      // println!("bbbbbb {}",parts.len() == user_parts.len());
                      // println!("{:?}",user_parts);
          
                      let mut billionaires = HashMap::new();
          
                      if methodb == #method {
                          // println!("{}",methodb);
                          if real_path == #path {
                              // println!("{}","billionairegreat");
                              // println!("{:?}",response);
                              let billionaire = GatesRequest {
                                  body: "",
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query(),
                              };
                              let response = {#fn_name(&billionaire)};
                              return bucket::zen(response,stream,header,compression,secure_header);
                                  // chunks.write_all(billionaire.0.as_bytes());
                                  // // chunks.write_all(&compress);
                                  // // println!("{:?}",b);
                          }
                          else if #path.contains("<") && #path.contains(">")  && !["/favicon.ico"].contains(&pathb) && parts.len() == user_parts.len() {
                              // #b;
                              // println!("{}","billionaires");
                              bucket::param_parser(parts,user_parts,&mut billionaires);
                              // println!("{:?}",billionaires);
                              let billionaire = GatesRequest {
                                  body: "",
                                  cookies: &response.cookie(),
                                  headers: response.header(),
                                  path: response.path(),
                                  method: response.method(),
                                  param: billionaires,
                                  query: response.query()
                                  };
                              let response = {#fn_name(&billionaire)};
                              bucket::zen(response,stream,header,compression,secure_header);
                          }
                      }
                  }
              }
              .into()
          }
      }
      }
    }
}
// Content-Length: {}\r\n\
// ; img-src 'self' url
// ; script-src 'self' url
// Strict-Transport-Security: max-age= ; includeSubDomains; preload
// Content-Encoding: br\r\n\


fn parse_attribute(params: String) -> (String, String,Option<Ident>) {
    let mut param = params.split(",");
    let method = param.nth(0).unwrap().replace(" ","");

    let mut method = method.split("=");
    let kind = method.nth(0).unwrap();
    let path = method.nth(0).unwrap().replace("\"", "");
    let path = path.trim();

    // println!("{:?}", kind);
    // println!("{}", path);

    let method_value:&str = match kind {
        "get" =>  "GET",
        "post" =>  "POST",
        "put" =>  "PUT",
        "delete" =>  "DELETE",
        "sse" =>  "text/event-stream",
        _ =>  "GET",
    };

    let path_value = if path.starts_with("/") {
        path
    } else {
        &format!("/{}", path)
    };

    let mut functions = "";
    let middleware = param.nth(0);
    if let Some(b) = middleware {
        let function = b.split("=").nth(1).unwrap();
        // println!("{}",function);
        functions = function;
    }

    let functions = parse_str::<Ident>(functions);
    match functions {
        Ok(functions) => {
            // println!("{}", method_value);
            // println!("{}", path_value);
            (method_value.to_string(), path_value.to_string(),Some(functions))
        }
        Err(_) => {
            // println!("{}", method_value);
            // println!("{}", path_value);
            (method_value.to_string(), path_value.to_string(),None)
        }
    }
    // println!("{}", method_value);
    // println!("{}", path_value);
    // (method_value.to_string(), path_value.to_string(),None)
}