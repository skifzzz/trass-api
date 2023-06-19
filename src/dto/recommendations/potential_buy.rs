use super::generic_info::GenericInfo;

use serde::Serialize;
use trass_core::data_access::data::recommendations::{Buy, RecommendedOperation};

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct PotentialBuy {
    #[serde(flatten)]
    generic_info: GenericInfo,
    loss: f64
}

impl From<RecommendedOperation<Buy>> for PotentialBuy {
    fn from(value: RecommendedOperation<Buy>) -> Self {
        PotentialBuy { generic_info: GenericInfo::from(value.info().clone()), loss: value.details().loss().clone() }
    }
}

impl From<&RecommendedOperation<Buy>> for PotentialBuy {
    fn from(value: &RecommendedOperation<Buy>) -> Self {
        PotentialBuy { generic_info: GenericInfo::from(value.info().clone()), loss: value.details().loss().clone() }
    }
}