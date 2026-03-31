use crate::helpers::float_to_string_for_hashing;

#[derive(Debug, Clone)]
pub struct ClientTwapRequest {
    pub asset: String,
    pub is_buy: bool,
    pub sz: f64,
    pub reduce_only: bool,
    pub duration_minutes: u32,
    pub randomize: bool,
}

impl ClientTwapRequest {
    pub(crate) fn sz_string(&self) -> String {
        float_to_string_for_hashing(self.sz)
    }
}

#[derive(Debug, Clone)]
pub struct ClientTwapCancelRequest {
    pub asset: String,
    pub twap_id: u64,
}
