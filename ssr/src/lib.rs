// #![recursion_limit = "256"]

#[macro_export]
macro_rules! page {
    ($main:expr,$billion_path:expr) => {
      {
        {
          fn b(main: &str, billion_path: &str) -> Option<String> {
            let bstruct = std::fs::read_dir(main);
            if let Ok(supers) = bstruct {
                for file in supers {
                    let directory = file.unwrap();
                    let d_path = directory.path();
                    let d_path = &d_path.to_str().unwrap().replace("\\", "/");
                    let bfile: Vec<&str> = d_path.split("/").collect();
                    let bpath: Vec<&str> = billion_path.split("/").collect();
                    // println!("{} {}", d_path, billion_path);
                    // app\billionaires\billionaire.html app/billionaires/billionaire.html
                    if d_path == billion_path {
                        // println!("{} {}",d_path,billion_path);
                        // println!("=========================== billionairegreathaRi{}", d_path);
                        // println!("{}", d_path);
                        let file = std::fs::read_to_string(d_path);
                        match file {
                            Err(error) => {
                                return Some("".to_string());
                            }
                            Ok(file) => {
                                return Some(file);
                            }
                        }
                    } else if d_path.contains("[") && d_path.contains("]") && bfile.len() == bpath.len() {
                        // println!("{}", "+++++++++++++++++++++++++++");
                        let mut proper = false;
                        // let mut billionaires = Vec::new();
                        for (b, file) in bfile.iter().enumerate() {
                            // println!("{:?}",proper);
                            // println!("{} {}",file,bpath[b]);
                            if *file != bpath[b] && file.starts_with("[") && file.ends_with("]") {
                                // println!("=================== {}",bpath[b]);
                                proper = true;
                                // proper.2 += 1;
                            } else if *file != bpath[b] && !file.starts_with("[") && !file.ends_with("]") {
                                // println!("billionaireeeeeeeeeeeeeeeeee {}",bpath[b]);
                                proper = false;
                                break;
                                // proper.1 += 1;
                            }
                        }
                        // println!("{:?}",proper);
                        if proper {
                            let billionairegreathari = format!("{}", d_path);
                            // println!("{}", billionairegreathari);
                            let book = std::fs::read_to_string(&billionairegreathari);
                            match book {
                                Err(err) => {
                                    println!("{}", err)
                                }
                                Ok(file) => {
                                    // println!("{}", file);
                                    return Some(file);
                                }
                            };
                            break;
                        }
                    } else {
                        if let Some(billionaires) = b(&d_path, billion_path) {
                            return Some(billionaires);
                        }
                    }
                }
            }
            None
        }
        b(&$main,&$billion_path)
        }
    }};
}

// use std::fs::read_to_string;

// fn b() {
//     let file = format!("{}/{}", "app", "billionaire");
//     let file = std::fs::read_to_string(file);
//     match file {
//         Err(err) => {
//             let b = format!(
//                 "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nCache-Control: no-cache\r\nConnection: keep-alive\r\nContent-Encoding: gzip\r\nContent-Length: {}\r\n\r\n{}",
//                 "billionaire".len(),
//                 "billionaire",
//             );
//             println!("{}", b);
//         }
//         Ok(file) => {
//             let b = format!(
//                 "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nCache-Control: no-cache\r\nConnection: keep-alive\r\nContent-Encoding: gzip\r\nContent-Length: {}\r\n\r\n{}",
//                 file.len(),
//                 file
//             );
//             println!("{}", b);
//         }
//     }
// }
