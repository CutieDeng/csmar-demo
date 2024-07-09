use std::{fs::File, io::{Read, Write}, path::Path, time};

use chrono::NaiveDate;

fn solve_p1<'a>(p: &Path, buf: &'a mut String) -> Vec<Vec<&'a str>> {
    assert! (p.is_file()); 
    buf.clear(); 
    let mut f = File::open(p).unwrap();  
    f.read_to_string(buf).unwrap(); 
    let bytes = buf.as_bytes(); 
    let mut bufcore = &buf[..]; 
    match bytes {
        &[239, 187, 191, ..] => {
            bufcore = &bufcore[3..]; 
        }, 
        _ => (), 
    }
    let buf2: Vec<_> = bufcore.char_indices().collect(); 
    let mut rst = Vec::new(); 
    let mut idx = 0; 
    let mut now = Vec::new(); 
    let mut start: usize = 0; 
    #[derive(PartialEq, Eq)]
    enum Quoted { None, Single, Double } 
    let mut quoted = Quoted::None; 
    loop {
        if idx >= buf2.len() { 
            break 
        } 
        match buf2[idx].1 {
            ',' => {
                match quoted {
                    Quoted::None => {
                        now.push(&bufcore[buf2[start].0..buf2[idx].0]); 
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
                        now.push(&bufcore[buf2[start].0..buf2[idx].0]); 
                        start = idx + 1; 
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
    if now.len() != 0 {
        println!("warning: last line without newline char. "); 
        rst.push(now); 
    }
    rst 
}

#[allow(unused)]
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

fn stock_code_to_u32(stkcd: &str) -> Option<u32> {
    let stk = pre_trim(stkcd)?; 
    stk.parse().ok()
}

fn pre_trim(s: &str) -> Option<&str> { 
    if let Some(stkcd) = s.strip_prefix('"') {
        stkcd.strip_suffix('"') 
    } else {
        Some(s)
    }
}

fn date_to_u16(trddt: &str) -> Option<u16> {
    let trddt = pre_trim(trddt)?; 
    let date = NaiveDate::parse_from_str(trddt, "%Y-%m-%d").ok()?;
    let base : NaiveDate = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(); 
    let dif = date - base; 
    let dif = dif.num_days(); 
    dif.try_into().ok()
}

fn float32_parse(opnprc: &str) -> Option<f32> {
    let opnprc = pre_trim(opnprc)?; 
    opnprc.parse().ok()
}

fn convert_u8(input: &[(u32, u16, f32)], output: &mut Vec<u8>) {
    for i in input {
        output.write_all(&i.0.to_le_bytes()).unwrap(); 
        output.write_all(&i.1.to_le_bytes()).unwrap(); 
        output.write_all(&i.2.to_le_bytes()).unwrap(); 
    }
}

fn main() {
    let p = Path::new("Resource");  
    let p2 = Path::new("b_res/db-jul-8.bin"); 
    if p2.exists() {
        println!("{:?} already exists, please delete it then execute the program for output. ", p2); 
        return ; 
    }
    let read_dir = p.read_dir().expect("Read 'stock dir'"); 
    let mut time0; 
    let mut s = String::new(); 
    let mut rst; 
    let mut rst2 = Vec::new(); 
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
        if false {
            for i in &rst[..10] {
                println!("{}", i.join("|")); 
            }
            return ; 
        }
        let mut idxes = [None; 3]; 
        for (i, v) in rst[0].iter().enumerate() {
            let j = match *v {
                "\"Stkcd\"" => 0, 
                r#""Trddt""# => 1, 
                r#""Opnprc""# => 2, 
                _ => {
                    if v.contains("Stkcd") {
                        println!("missing select: {}|", v); 
                        println!("why? "); 
                        println!("v content: {:?}", v.as_bytes()); 
                        println!("stkcd: {:?}", r#""Stkcd""#.as_bytes()); 
                    }
                    continue; 
                }
            }; 
            assert!(idxes[j].is_none()); 
            idxes[j] = Some(i); 
        }
        match idxes {
            [Some(s), Some(t), Some(o)] => {
                for piece in &rst[1..] { 
                    let s = stock_code_to_u32(piece[s]); 
                    let t = date_to_u16(piece[t]); 
                    let o = float32_parse(piece[o]); 
                    match (s, t, o) {
                        (Some(s), Some(t), Some(o)) => {
                            rst2.push((s, t, o)); 
                        }
                        _ => {
                            println!("parse failed, (stkcd, trddt, opnprc): {:?}", (s, t, o));  
                            continue; 
                        }
                    }
                }
            }, 
            _ => {
                println!("failed to find the properties in file name[{}], idxes={:?}", rname, idxes); 
                continue; 
            }
        }
    }
    let convert_time = time::Instant::now(); 
    let mut rst3 = Vec::new(); 
    convert_u8(&rst2[..], &mut rst3); 
    println!("convert objs time: {}s", convert_time.elapsed().as_secs_f32()); 
    let write_time = time::Instant::now(); 
    let mut of = File::create_new(p2).unwrap(); 
    of.write_all(&rst3).unwrap(); 
    drop(of); 
    println!("write objs time: {}s", write_time.elapsed().as_secs_f32()); 
    println!("end of writing, with {} objects. ", rst2.len()); 
}
