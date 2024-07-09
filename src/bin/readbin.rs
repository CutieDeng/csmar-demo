use std::{fs::File, io::Read, path::Path, time::Instant};

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
    println!("read op"); 
    let mut f = File::open(p).unwrap(); 
    let mut buf = Vec::new(); 
    f.read_to_end(&mut buf).unwrap(); 
    assert!(buf.len() % 10 == 0, "Unexpected file length[{}] = {}", p.as_os_str().to_string_lossy(), buf.len()); 
    let mut data = Vec::new();

    convert_data(&buf, &mut data); 
    println!("convert time cost: {}s", start.elapsed().as_secs_f32()); 

    let analyze_time = Instant::now(); 
    let highest = data.iter().map(|d| d.open_price).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap(); 
    let lowest = data.iter().map(|d| d.open_price).min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap(); 
    let first_date = data.iter().map(|d| d.trade_date_by_1970).min().unwrap(); 
    let last_date = data.iter().map(|d| d.trade_date_by_1970).max().unwrap(); 
    println!("analyze time cost: {}s", analyze_time.elapsed().as_secs_f32()); 

    println!("Some analysis data: "); 
    println!("data size: {}", data.len()); 
    println!("the highest price: {}", highest); 
    println!("the lowest  price: {}", lowest); 
    println!("the first day: {}", first_date); 
    println!("the last  day: {}", last_date); 
}

