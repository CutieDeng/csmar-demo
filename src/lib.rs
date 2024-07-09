use ndarray::Array2;

pub mod stock; 

pub fn read_matrix(content: &[u8]) -> (Array2<f32>, &[u8]) {
    let mut ar = [0; 8]; 
    ar.copy_from_slice(&content[..8]); 
    let row = u64::from_le_bytes(ar) as usize; 
    ar.copy_from_slice(&content[8..16]); 
    let column = u64::from_le_bytes(ar) as usize; 
    let len = row * column; 
    let content1 = &content[16..]; 
    let mut ar = [0; 4]; 
    let mut v = Vec::new(); 
    let mut c1 = content1; 
    for _ in 0..len {
        ar.copy_from_slice(&c1[..4]); 
        c1 = &c1[4..]; 
        v.push(f32::from_le_bytes(ar)); 
    }
    let rst = Array2::from_shape_vec((row, column), v).unwrap(); 
    (rst, c1)
}