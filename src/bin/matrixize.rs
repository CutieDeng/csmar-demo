use std::path::Path;
use std::time::Instant;
use std::io::{BufWriter, Read, Write};
use std::fs::File;

use ndarray::{s, Array2};

fn main() {
    let start_time = Instant::now(); 
    let p = Path::new("b_res/db-jul-8.bin2"); 
    let p2 = Path::new("b_res/db-jul-8.bin3"); 
    let outf = File::create_new(p2).expect("create file for output"); 
    println!("matrixize"); 
    let mut inf = File::open(p).unwrap(); 
    let mut input_buffer0 = Vec::new(); 
    inf.read_to_end(&mut input_buffer0).unwrap(); 
    let mut ar = [0; 8]; 
    ar.copy_from_slice(&input_buffer0[0..8]); 
    let len = u64::from_le_bytes(ar) as usize; 
    let data = handle(len, &input_buffer0[8..]); 
    println!("parse line to 2d array: {}s", start_time.elapsed().as_secs_f32()); 
    println!("data shape: {:?}", data.shape()); 
    // 10 lines: 
    println!("10 lines data: {}", data.slice(s![..10, ..])); 
    let sh = data.shape(); 
    assert_eq!(sh.len(), 2); 
    let write_instant = Instant::now(); 
    let mut bw = BufWriter::new(outf); 
    bw.write_all(&sh[0].to_le_bytes()).unwrap(); 
    bw.write_all(&sh[1].to_le_bytes()).unwrap();  
    let floats = data.as_slice().unwrap();
    for f in floats {
        bw.write_all(&f.to_le_bytes()).unwrap(); 
    }
    drop(bw); 
    println!("write done, finish in {}s", write_instant.elapsed().as_secs_f32()); 
    println!("write {} bytes content in file {:?}", 16 + floats.len() * 4, p2.as_os_str().to_string_lossy()); 
}

fn calc_size(mut len: usize, buf: &[u8], window_size: usize) -> usize {
    let mut buf = buf; 
    let mut ar = [0; 8]; 
    let mut cnt = 0; 
    loop {
        if len == 0 {
            break; 
        }
        len -= 1; 
        // ignore the first 4 bytes (u32), as the stock code 
        buf = &buf[4..]; 
        // then 8 bytes are the len of a sub slice of this stock 
        ar.copy_from_slice(&buf[..8]); 
        let len = u64::from_le_bytes(ar) as usize; 
        buf = &buf[8..]; 
        buf = &buf[4 * len ..];     
        if len >= window_size {
            cnt += len - window_size + 1; 
        }
    }
    cnt 
}

fn handle(mut len: usize, buf: &[u8]) -> Array2<f32> {
    let wsize = 61; 
    let row_cnt = calc_size(len, buf, wsize); 
    let rst; 
    let mut rst0 = Vec::with_capacity(wsize * row_cnt); 
    // ignore the first 4 bytes (u32), as the stock code 
    let mut buf = buf; 
    let mut ar = [0; 8]; 
    let mut ar4 = [0; 4]; 
    let mut values = Vec::new(); 
    loop {
        if len == 0 {
            break; 
        }
        len -= 1; 
        buf = &buf[4..]; 
        // then 8 bytes are the len of a sub slice of this stock 
        ar.copy_from_slice(&buf[..8]); 
        let len = u64::from_le_bytes(ar) as usize; 
        buf = &buf[8..]; 
        values.clear(); 
        for _ in 0..len {
            ar4.copy_from_slice(&buf[..4]); 
            buf = &buf[4..]; 
            values.push(f32::from_le_bytes(ar4)); 
        }
        for w in values.windows(wsize) {
            rst0.extend_from_slice(w); 
        }
    }
    rst = Array2::from_shape_vec((row_cnt, wsize), rst0).unwrap(); 
    rst 
}