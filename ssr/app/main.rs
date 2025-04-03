use std::fs::{self, read_dir};

use sergy::page;

// use ssr::page;

fn main() {
    // let b = [0, 1, 2, 3, 4];
    // let path = "";
    // let bpath = "";
    // fn billionaire(bpath: &str, billion_path: &str) -> Option<String> {
    //     let b = read_dir(bpath);
    //     if let Ok(b) = b {
    //         for i in b {
    //             if let Ok(b) = i {
    //                 let d_path = b.path();
    //                 let d_path = &d_path.to_str().unwrap().replace("\\", "/");
    //                 println!("{}", d_path);
    //                 if d_path == billion_path {
    //                     return Some(d_path.to_string());
    //                 } else {
    //                     if let Some(billionaire) = billionaire(d_path, billion_path) {
    //                         return Some(billionaire);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    // let billionaires = billionaire("app", "app/billionaires");
    let billionaire = page!("app", "app/b/b/b/b.html");
    println!("{}", billionaire.unwrap());
    // println!("{:?}", billionaires);
    // let path = "app/b/explore/b.html";
    // let b = b("app", path).unwrap();
    // println!("{}", b);
    // println!("Hello, world!");
}

fn b(main: &str, billion_path: &str) -> Option<String> {
    let bstruct = read_dir(main);
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
                // println!("=========================== billionairegreathaRi{}", d_path);
                // println!("{}", d_path);
                let file = fs::read_to_string(d_path);
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
                    if *file != bpath[b] && file.starts_with("[") && file.ends_with("]") {
                        proper = true;
                    } else if *file != bpath[b] && !file.starts_with("[") && !file.ends_with("]") {
                        proper = false;
                    }
                }
                if proper {
                    let billionairegreathari = format!("{}", d_path);
                    // println!("{}", billionairegreathari);
                    let book = fs::read_to_string(&billionairegreathari);
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
