pub mod historic_data;
pub mod not_enough_data;
pub mod generic_info;
pub mod potential_buy;
pub mod potential_sell;
pub mod reduce_price_buy;
pub mod stop_sell;

use serde::Serialize;
use trass_core::data_access::data::recommendations::OperationContainer;

use self::{not_enough_data::NotEnoughData, potential_buy::PotentialBuy, potential_sell::PotentialSell, reduce_price_buy::ReducePriceBuy, stop_sell::StopSell};

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct Recommendations {
    potential_buy:Vec<PotentialBuy>,
    reduce_price_buy:Vec<ReducePriceBuy>,
    potential_sell:Vec<PotentialSell>,
    sell_with_stop:Vec<StopSell>,
    not_enough_data:Vec<NotEnoughData>
}

impl From<OperationContainer> for Recommendations {
    fn from(value: OperationContainer) -> Self {
        let potential_buy = value.buy().into_iter().map(PotentialBuy::from).collect();
        let reduce_price_buy = value.buy_more_than_10().into_iter().map(ReducePriceBuy::from).collect();
        let potential_sell = value.sell().into_iter().map(PotentialSell::from).collect();
        let sell_with_stop = value.sell_more_than_10().into_iter().map(StopSell::from).collect();
        let not_enough_data = value.unknown().into_iter().map(NotEnoughData::from).collect();

        Recommendations { potential_buy, 
            reduce_price_buy, 
            potential_sell, 
            sell_with_stop,
            not_enough_data,
        }
    }
}



