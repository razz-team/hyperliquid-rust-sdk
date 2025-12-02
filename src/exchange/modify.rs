use serde::{Deserialize, Serialize};
use super::ids::OidOrCloidTrait;

use super::{order::OrderRequest, ClientOrderRequest};

#[derive(Debug)]
pub struct ClientModifyRequest<T: OidOrCloidTrait> {
    pub oid: T,
    pub order: ClientOrderRequest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModifyRequest<T: OidOrCloidTrait> {
    pub oid: T,
    pub order: OrderRequest,
}
