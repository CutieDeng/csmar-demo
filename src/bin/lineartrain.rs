use std::{fs::File, io::Read, path::Path, time::Instant};

use csmar_parse::read_matrix;
use ndarray::Array2;
use rand::random;

fn main() {
    let rtime = Instant::now(); 
    let p = Path::new("b_res/db-jul-8.bin3"); 
    let p2 = Path::new("b_res/db-jul-8.bin4"); 
    let mut inf = File::open(p).unwrap(); 
    let mut buf = Vec::new(); 
    inf.read_to_end(&mut buf).unwrap(); 
    let (nda, _) = read_matrix(&buf); 
    // attempt to find a result ... for it to meet 0? 
    // let mut ra = rand::thread_rng(); 
    let mut w = Array2::from_shape_fn((nda.shape()[1], 1), |_| rand::random::<f32>()); 
    let mut b : f32 = random();
    println!("read end; time cost: {}s", rtime.elapsed().as_secs_f32());  
    // find ans 
    let rate : f32 = 1e-6; 
    let mut cnt = 0; 
    let mut t = Instant::now(); 
    let mut enver = false; 
    loop {
        let y = nda.dot(&w) + b; 
        let y2 = y.map(|v| v * v); 
        let s = y2.sum() / y2.len() as f32; 
        if !enver {
            enver = true; 
            println!("|  start, entropy: {}", s); 
        }
        cnt += 1; 
        if t.elapsed().as_secs() >= 1 {
            println!("|{cnt} times, entropy: {}", s); 
            t = Instant::now(); 
        }
        b -= y.sum() / y.len() as f32 * rate; 
        let mut w2 = nda.t().dot(&y);
        w2.map_inplace(|v| *v = *v * rate); 
        w = &w - &w2; 
    }
}

