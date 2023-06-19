use super::generic_info::GenericInfo;

use serde::Serialize;
use trass_core::data_access::data::recommendations::{RecommendedOperation, SellMoreThan10};

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct StopSell {
    #[serde(flatten)]
    generic_info: GenericInfo,
    stop_price: f64,
    gain_on_stop: f64,
}

impl From<RecommendedOperation<SellMoreThan10>> for StopSell {
    fn from(value: RecommendedOperation<SellMoreThan10>) -> Self {
        StopSell { 
            generic_info: GenericInfo::from(value.info().clone()), 
            stop_price: value.details().stop_price().clone(), 
            gain_on_stop: value.details().gain_on_stop().clone(), 
        }
    }
}

impl From<&RecommendedOperation<SellMoreThan10>> for StopSell {
    fn from(value: &RecommendedOperation<SellMoreThan10>) -> Self {
        StopSell { 
            generic_info: GenericInfo::from(value.info().clone()), 
            stop_price: value.details().stop_price().clone(), 
            gain_on_stop: value.details().gain_on_stop().clone(), 
        }
    }
}