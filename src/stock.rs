#[derive(Clone, Copy)]
pub struct TradeData {
    pub stock_id: u32, 
    pub trade_date_by_1970: u16, 
    pub open_price: f32, 
}