use std::{collections::HashMap, fs::File, io::{Read, Write}, path::Path, time::Instant};

use csmar_parse::stock::TradeData;

fn convert_data(bytes: &[u8], output: &mut Vec::<TradeData>) {
    let chunks = bytes.chunks_exact(10); 
    let mut array = [0u8; 4]; 
    let mut array2 = [0; 2]; 
    for chunk in chunks {
        array.copy_from_slice(&chunk[0..4]); 
        let stock_id = u32::from_le_bytes(array); 
        array2.copy_from_slice(&chunk[4..6]); 
        let trade_date_by_1970 = u16::from_le_bytes(array2); 
        array.copy_from_slice(&chunk[6..]); 
        let open_price = f32::from_le_bytes(array); 
        output.push(TradeData { stock_id, trade_date_by_1970, open_price }); 
    }
    return ; 
}

fn main() {

    let start = Instant::now();
    let p = Path::new("b_res/db-jul-8.bin"); 
    let p2 = Path::new("b_res/db-jul-8.bin2"); 

    let mut of = File::create_new(p2).expect("Delete the file to make me happy, for safely output my content"); 

    println!("read op"); 
    let mut f = File::open(p).unwrap(); 
    let mut buf = Vec::new(); 
    f.read_to_end(&mut buf).unwrap(); 
    assert!(buf.len() % 10 == 0, "Unexpected file length[{}] = {}", p.as_os_str().to_string_lossy(), buf.len()); 
    let mut data = Vec::new();

    convert_data(&buf, &mut data); 
    println!("convert time cost: {}s", start.elapsed().as_secs_f32()); 

    let to_hash = Instant::now(); 
    let mut cd_map = HashMap::new(); 
    for d in &data {
        let en = cd_map.entry(d.stock_id).or_insert_with(|| Vec::new());
        en.push(*d); 
    }
    for cd in &mut cd_map {
        cd.1.sort_unstable_by(|l, r| l.trade_date_by_1970.cmp(&r.trade_date_by_1970)); 
    }
    println!("sort the datetime, time cost: {}s", to_hash.elapsed().as_secs_f32()); 

    let serialize_time = Instant::now(); 
    let mut output = Vec::new(); 
    let mut rates = Vec::new(); 
    
    output.write_all(&(cd_map.len() as u64).to_le_bytes()).unwrap(); 
    for cd in &cd_map {
        let ex = cd.1.windows(2); 
        rates.clear(); 
        for e in ex {
            let e0 = e[0].open_price; 
            let e1 = e[1].open_price; 
            let dif = e1 / e0 - 1.0; 
            rates.push(dif); 
        }
        let s: u64 = rates.len() as _; 
        output.write_all(&cd.0.to_le_bytes()).unwrap();  
        output.write_all(&s.to_le_bytes()).unwrap();  
        for r in &rates {
            output.write_all(&r.to_le_bytes()).unwrap();  
        }
    }

    of.write_all(&output).unwrap();  
    drop(of); 

    println!("write data time: {}s", serialize_time.elapsed().as_secs_f32()); 
}

