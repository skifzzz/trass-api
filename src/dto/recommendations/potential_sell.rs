use super::generic_info::GenericInfo;

use serde::Serialize;
use trass_core::data_access::data::recommendations::{Sell, RecommendedOperation};

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct PotentialSell {
    #[serde(flatten)]
    generic_info: GenericInfo,
    gain: f64
}

impl From<RecommendedOperation<Sell>> for PotentialSell {
    fn from(value: RecommendedOperation<Sell>) -> Self {
        PotentialSell { generic_info: GenericInfo::from(value.info().clone()), gain: value.details().gain().clone() }
    }
}

impl From<&RecommendedOperation<Sell>> for PotentialSell {
    fn from(value: &RecommendedOperation<Sell>) -> Self {
        PotentialSell { generic_info: GenericInfo::from(value.info().clone()), gain: value.details().gain().clone() }
    }
}