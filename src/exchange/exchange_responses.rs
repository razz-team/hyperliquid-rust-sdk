use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RestingOrder {
    pub oid: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilledOrder {
    pub total_sz: String,
    pub avg_px: String,
    pub oid: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ExchangeDataStatus {
    Success,
    WaitingForFill,
    WaitingForTrigger,
    Error(String),
    Resting(RestingOrder),
    Filled(FilledOrder),
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExchangeDataStatuses {
    pub statuses: Vec<ExchangeDataStatus>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TwapRunning {
    pub twap_id: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TwapStatus {
    Running(TwapRunning),
    Error(String),
    Success,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TwapDataStatus {
    pub status: TwapStatus,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ExchangeResponse {
    #[serde(rename = "order")]
    Order { data: Option<ExchangeDataStatuses> },
    #[serde(rename = "cancel")]
    Cancel { data: Option<ExchangeDataStatuses> },
    #[serde(rename = "twapOrder")]
    TwapOrder { data: Option<TwapDataStatus> },
    #[serde(rename = "twapCancel")]
    TwapCancel { data: Option<TwapDataStatus> },
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "status", content = "response")]
pub enum ExchangeResponseStatus {
    Ok(ExchangeResponse),
    Err(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_twap_order_response() {
        let json = r#"{"status":"ok","response":{"type":"twapOrder","data":{"status":{"running":{"twapId":77738308}}}}}"#;
        let resp: ExchangeResponseStatus = serde_json::from_str(json).unwrap();
        match resp {
            ExchangeResponseStatus::Ok(ExchangeResponse::TwapOrder { data }) => {
                assert!(matches!(
                    data.unwrap().status,
                    TwapStatus::Running(TwapRunning { twap_id: 77738308 })
                ));
            }
            _ => panic!("expected TwapOrder"),
        }
    }

    #[test]
    fn deserialize_twap_cancel_response() {
        let json = r#"{"status":"ok","response":{"type":"twapCancel","data":{"status":"success"}}}"#;
        let resp: ExchangeResponseStatus = serde_json::from_str(json).unwrap();
        match resp {
            ExchangeResponseStatus::Ok(ExchangeResponse::TwapCancel { data }) => {
                assert!(matches!(data.unwrap().status, TwapStatus::Success));
            }
            _ => panic!("expected TwapCancel"),
        }
    }

    #[test]
    fn deserialize_twap_error_response() {
        let json = r#"{"status":"ok","response":{"type":"twapOrder","data":{"status":{"error":"Invalid TWAP duration: 1 min(s)"}}}}"#;
        let resp: ExchangeResponseStatus = serde_json::from_str(json).unwrap();
        match resp {
            ExchangeResponseStatus::Ok(ExchangeResponse::TwapOrder { data }) => {
                assert!(matches!(
                    data.unwrap().status,
                    TwapStatus::Error(e) if e == "Invalid TWAP duration: 1 min(s)"
                ));
            }
            _ => panic!("expected TwapOrder"),
        }
    }

    #[test]
    fn deserialize_order_response() {
        let json = r#"{"status":"ok","response":{"type":"order","data":{"statuses":[{"resting":{"oid":123}}]}}}"#;
        let resp: ExchangeResponseStatus = serde_json::from_str(json).unwrap();
        match resp {
            ExchangeResponseStatus::Ok(ExchangeResponse::Order { data }) => {
                let statuses = &data.unwrap().statuses;
                assert_eq!(statuses.len(), 1);
                assert!(matches!(&statuses[0], ExchangeDataStatus::Resting(o) if o.oid == 123));
            }
            _ => panic!("expected Order"),
        }
    }

    #[test]
    fn deserialize_cancel_response() {
        let json = r#"{"status":"ok","response":{"type":"cancel","data":{"statuses":["success"]}}}"#;
        let resp: ExchangeResponseStatus = serde_json::from_str(json).unwrap();
        match resp {
            ExchangeResponseStatus::Ok(ExchangeResponse::Cancel { data }) => {
                assert!(matches!(&data.unwrap().statuses[0], ExchangeDataStatus::Success));
            }
            _ => panic!("expected Cancel"),
        }
    }
}
