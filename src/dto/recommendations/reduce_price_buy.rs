use super::generic_info::GenericInfo;

use serde::Serialize;
use trass_core::data_access::data::recommendations::{RecommendedOperation, BuyMoreThan10};

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct ReducePriceBuy {
    #[serde(flatten)]
    generic_info: GenericInfo,
    amount: f64,
    total_cost:f64
}

impl From<RecommendedOperation<BuyMoreThan10>> for ReducePriceBuy {
    fn from(value: RecommendedOperation<BuyMoreThan10>) -> Self {
        ReducePriceBuy { generic_info: GenericInfo::from(value.info().clone()), 
            amount: value.details().amount().clone(),
            total_cost: value.details().spend_total().clone()
         }
    }
}

impl From<&RecommendedOperation<BuyMoreThan10>> for ReducePriceBuy {
    fn from(value: &RecommendedOperation<BuyMoreThan10>) -> Self {
        ReducePriceBuy { generic_info: GenericInfo::from(value.info().clone()), 
            amount: value.details().amount().clone(),
            total_cost: value.details().spend_total().clone()
         }
    }
}