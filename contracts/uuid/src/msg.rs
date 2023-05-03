use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[cfg_attr(test, derive(Default))]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(UuidResponse)]
    GetUuids { amount: u8, entropy: u32 }
}

#[cw_serde]
pub struct UuidResponse {
    pub uuids: Vec<String>
}