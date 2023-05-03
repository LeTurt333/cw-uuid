#[cfg(not(feature = "library"))]
use crate::{
    msg::*, error::*
};
use cosmwasm_std::{
    entry_point, to_binary, DepsMut, Env,
    MessageInfo, Response, StdResult, Binary,
};
use cw2::set_contract_version;
use cw_uuid::get_uuid;

const CONTRACT_NAME: &str = "crates.io:cw-uuid";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("Instantiate", "cw-uuid")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetUuids { amount, entropy } => to_binary(&get_uuids(amount, env.block.height, entropy)?)
    }
}

fn get_uuids(amount: u8, block_height: u64, entropy: u32) -> StdResult<UuidResponse> {

    let uuids: Vec<String> = (0..amount).map(|i| {
        get_uuid(block_height, entropy + i as u32)
    }).collect();

    Ok(UuidResponse { uuids })
}


#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        let a = true;
        assert_eq!(a, true);
    }
}

