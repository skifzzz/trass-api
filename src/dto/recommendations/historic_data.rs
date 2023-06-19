use trass_core::data_access::data::token_info::TokenInfo;

use serde::Serialize;

#[derive(Serialize)]
pub struct HistoricData {
    price: f64,
    amount: f64,
    total: f64,
}

impl From<TokenInfo> for HistoricData {
    fn from(value: TokenInfo) -> Self {
        HistoricData { price: *value.price(), 
            amount: *value.amount(), 
            total: *value.total() }
    }
}

// impl From<Vec<TokenInfo>> for Vec<HistoricData> {
//     fn from(value: Vec<TokenInfo>) -> Self {
//         value.into_iter().map(HistoricData::from).collect()
//     }
// }