use std::{fs::File, io::Read, path::Path, time};

fn get_path() -> &'static Path {
    let path = env!("CSMAR");
    let path = Path::new(path); 
    path 
}

fn solve_p1<'a>(p: &Path, buf: &'a mut String) -> Vec<Vec<&'a str>> {
    assert! (p.is_file()); 
    let mut f = File::open(p).unwrap();  
    f.read_to_string(buf).unwrap(); 
    let buf2: Vec<_> = buf.char_indices().collect(); 
    let mut rst = Vec::new(); 
    let mut idx = 0; 
    let mut now = Vec::new(); 
    let mut start: usize = 0; 
    #[derive(PartialEq, Eq)]
    enum Quoted { None, Single, Double } 
    let mut quoted = Quoted::None; 
    loop {
        if idx >= buf2.len() { break } 
        match buf2[idx].1 {
            ',' => {
                match quoted {
                    Quoted::None => {
                        now.push(&buf[buf2[start].0..buf2[idx].0]); 
                        start = idx + 1; 
                    } 
                    _ => (), 
                }
            }, 
            '\'' => {
                match quoted {
                    Quoted::None => quoted = Quoted::Single, 
                    Quoted::Single => quoted = Quoted::None, 
                    Quoted::Double => (), 
                }
            }, 
            '"' => {
                match quoted {
                    Quoted::None => quoted = Quoted::Double, 
                    Quoted::Double => quoted = Quoted::None, 
                    Quoted::Single => (), 
                }
            }, 
            '\n' => {
                match quoted {
                    Quoted::None => {
                        rst.push(now);  
                        now = Vec::new(); 
                    }
                    _ => (), 
                }
            }, 
            _ => {} 
        }
        idx += 1; 
    }
    rst 
}

fn solve_p0(p: &Path) -> Vec<Vec<String>> {
    assert! (p.is_file()); 
    let mut f = File::open(p).unwrap();  
    let mut buf = String::new(); 
    f.read_to_string(&mut buf).unwrap(); 
    let buf2: Vec<_> = buf.char_indices().collect(); 
    let mut rst = Vec::new(); 
    let mut idx = 0; 
    let mut now = Vec::new(); 
    let mut start: usize = 0; 
    #[derive(PartialEq, Eq)]
    enum Quoted { None, Single, Double } 
    let mut quoted = Quoted::None; 
    loop {
        if idx >= buf2.len() { break } 
        match buf2[idx].1 {
            ',' => {
                match quoted {
                    Quoted::None => {
                        now.push(buf[buf2[start].0..buf2[idx].0].to_string());  
                        start = idx + 1; 
                    } 
                    _ => (), 
                }
            }, 
            '\'' => {
                match quoted {
                    Quoted::None => quoted = Quoted::Single, 
                    Quoted::Single => quoted = Quoted::None, 
                    Quoted::Double => (), 
                }
            }, 
            '"' => {
                match quoted {
                    Quoted::None => quoted = Quoted::Double, 
                    Quoted::Double => quoted = Quoted::None, 
                    Quoted::Single => (), 
                }
            }, 
            '\n' => {
                match quoted {
                    Quoted::None => {
                        rst.push(now);  
                        now = Vec::new(); 
                    }
                    _ => (), 
                }
            }, 
            _ => {} 
        }
        idx += 1; 
    }
    rst 
}

fn main() {
    let p = get_path(); 
    let read_dir = p.read_dir().expect("Read 'stock dir'"); 
    let mut time0; 
    let mut s = String::new(); 
    let mut rst; 
    for r in read_dir {
        s.clear(); 
        let r = r.unwrap(); 
        let rname = r.file_name(); 
        let rname = rname.to_string_lossy(); 
        if !rname.ends_with(".csv") {
            println!("Ignore {}", rname); 
            continue ; 
        }
        let p = r.path(); 
        time0 = time::Instant::now(); 
        // let rst = solve_p0(&p); 
        rst = solve_p1(&p, &mut s); 
        let duration = time0.elapsed(); 
        println!("line: {}", rst.len());  
        println!("time cost: {:.4}", duration.as_secs_f64()); 
    }
}
