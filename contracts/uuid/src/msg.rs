use cosmwasm_schema::{cw_serde, QueryResponses};
use nois::NoisCallback;

#[cw_serde]
#[cfg_attr(test, derive(Default))]
pub struct InstantiateMsg {
    pub nois_proxy: String
}

#[cw_serde]
pub enum ExecuteMsg {
    GetUUIDs { num_uuid: u8, entropy: Option<String>},
    // Nois Receiver
    NoisReceive{callback: NoisCallback},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(NoisProxyResponse)]
    GetNoisProxy{}

}

#[cw_serde]
pub struct NoisProxyResponse {
    pub nois_proxy: String
}

// #[derive(Serialize, Deserialize, JsonSchema)]
// pub struct MigrateMsg {}