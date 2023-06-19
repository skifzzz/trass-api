use trass_core::data_access::data::recommendations::{RecommendedOperation, Unknown};

use super::generic_info::GenericInfo;

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct NotEnoughData {
    #[serde(flatten)]
    generic_info: GenericInfo
}

impl From<RecommendedOperation<Unknown>> for NotEnoughData {
    fn from(value: RecommendedOperation<Unknown>) -> Self {
        NotEnoughData { generic_info: GenericInfo::from(value.info().clone()) }
    }
}

impl From<&RecommendedOperation<Unknown>> for NotEnoughData {
    fn from(value: &RecommendedOperation<Unknown>) -> Self {
        NotEnoughData { generic_info: GenericInfo::from(value.info().clone()) }
    }
}