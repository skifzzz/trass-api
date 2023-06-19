use trass_core::data_access::data::recommendations::Recommendation;

use super::historic_data::HistoricData;

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct GenericInfo {
    token: String,
    current_amount: Option<f64>,
    current_price: Option<f64>,
    historic_data: Option<HistoricData>,
}

impl From<Recommendation> for GenericInfo {
    fn from(value: Recommendation) -> Self {
        GenericInfo { token: value.token(), 
            current_amount: value.current_amount(), 
            current_price: value.current_price(), 
            historic_data: value.historic_data().map(HistoricData::from) }
    }
}