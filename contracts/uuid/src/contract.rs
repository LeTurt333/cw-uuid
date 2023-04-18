#[cfg(not(feature = "library"))]
use crate::{
    msg::*, state::*, error::*
};
use cosmwasm_std::{
    coin, entry_point, to_binary, DepsMut, Env,
    MessageInfo, Response, Addr, WasmMsg, StdResult, 
    Coin, Uint128, Binary, Deps,
};

use cw2::set_contract_version;

use nois::{
    NoisCallback, ProxyExecuteMsg,
    sub_randomness_with_key as sub_rand,
    //MAX_JOB_ID_LEN
};

use uuid::{Builder};

// Contract name used for migration
const CONTRACT_NAME: &str = "crates.io:cw-uuid";
// Contract version thats used for migration
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const FEE_DENOM: &str = "ujunox";
// 1 juno
const FEE_AMOUNT: u32 = 1_000_000;

//const MAX_UUIDS: u8 = 100;
const MAX_UUIDS: u8 = 1;

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Instantiate
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let nois_proxy_addr = deps
        .api
        .addr_validate(&msg.nois_proxy)
        .map_err(|_| ContractError::GenericError("Invalid Nois Proxy".to_string()))?;

    NOIS_PROXY.save(deps.storage, &nois_proxy_addr)?;

    Ok(Response::new()
        .add_attribute("Instantiate", "cw-uuid")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::GetUUIDs { num_uuid, entropy } => get_uuids(deps, info, num_uuid, entropy),
        // Nois Callback
        ExecuteMsg::NoisReceive { callback } => execute_nois_callback(deps, env, info, callback),
    }
}

fn execute_nois_callback(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    callback: NoisCallback
) -> Result<Response, ContractError> {

    //-----------------------------------
    // Verify Nois Proxy is sender
    //-----------------------------------
    let nois: Addr = NOIS_PROXY.load(deps.storage)?;
    if info.sender != nois {
        return Err(ContractError::Unauthorized {});
    }

    //-----------------------------------
    // If randomness is new, update randomness
    //-----------------------------------
    let old_rand = STATE_RANDOMNESS.load(deps.storage)?;

    let new_rand: [u8; 32] = callback
        .randomness
        .to_array()
        .map_err(|_e| ContractError::GenericError("Invalid Randomness".to_string()))?;

    if old_rand != new_rand {
        return Ok(Response::new().add_attribute("Same randomness", "Not updated"));
    }

    STATE_RANDOMNESS.update(deps.storage, |_old| -> Result<[u8; 32], cosmwasm_std::StdError> {
        Ok(new_rand)
    }).map_err(|_e| ContractError::GenericError("Error updating randomness".to_string()))?;

    Ok(Response::new().add_attribute("Randomness updated", "New"))

}


fn get_uuids<T: Sized + AsRef<[u8]>> (
    deps: DepsMut,
    info: MessageInfo,
    num_uuid: u8, 
    entropy: Option<T>,
) -> Result<Response, ContractError> {

    // If message doesn't have fee amount, error
    check_fee(info.funds)?;

    // Check number of UUID's requested is <= max
    if num_uuid > MAX_UUIDS {
        return Err(ContractError::GenericError(format!("Requested UUIDS: {} - Max UUIDS: {}", num_uuid, MAX_UUIDS)));
    }

    // Load current randomness & incrementor
    let base_rand = STATE_RANDOMNESS.load(deps.storage)?;
    let count = INCREMENT.load(deps.storage)?;
    INCREMENT.increment(deps.storage)?;

    // Send GetNextRandomnessMsg to Nois Proxy
    let nois_msg = ProxyExecuteMsg::GetNextRandomness { 
        job_id: count.to_string()
    };

    // Add query for fee amount once github is back up
    let execute = WasmMsg::Execute {
        contract_addr: NOIS_PROXY.load(deps.storage)?.into(),
        msg: to_binary(&nois_msg)?,
        funds: vec![coin(150, "ujunox")]
    };

    // If entropy is passed, use it
    let mut provider = match entropy {
        None => sub_rand(base_rand, count.to_string()),
        Some(x) => {
            let first = sub_rand(base_rand, count.to_string()).provide();
            sub_rand(first, x)
        }
    };

    let get_uuids: Result<Vec<[u8; 16]>, ContractError> = (0..num_uuid).map(|_i| {
        // Get 32 bytes
        let rand = provider.provide();

        // take middle 16 bytes
        let bytes = <[u8; 16]>::try_from(&rand[8..24])
            .map_err(|_e| ContractError::GenericError("Converting Slice to Array".to_string()))?;

        // use in UUID generator
        let uuid = Builder::from_random_bytes(bytes).into_uuid();

        // convert to bytes via uuid.into_bytes
        // How should send large huge of message.data?
        // Create one big slice & separate each UUID with specific char? X
        // Limit to 3 and always send 3 UUID's? eh
        // limit to 1 and always send 1 as Binary::from(uuid.to_string())? eh^2
        // Publish package with methods to take Vec<[u8; 16]> and convert back into UUIDs? not bad
        return Ok(uuid.into_bytes());

    }).collect();

    // Currently limiting to 1 UUID until above question is decided
    let uuids = get_uuids?;
    if uuids.len() > 1 {
        return Err(ContractError::GenericError("Temporary over 1 UUID error".to_string()));
    }

    let b: Binary = Binary::from(&uuids[0]);

    Ok(Response::new().add_message(execute).set_data(b))
}


fn check_fee(
    funds: Vec<Coin>
) -> Result<(), ContractError> {

    // Only 1 coin 
    if funds.len() != 1 {
        return Err(ContractError::GenericError("Invalid Fee Coins".to_string()));
    }

    // Correct denom
    if funds[0].denom != FEE_DENOM {
        return Err(ContractError::GenericError("Invalid Fee Denom".to_string()));
    }

    // Correct amount
    if funds[0].amount != Uint128::from(FEE_AMOUNT) {
        return Err(ContractError::GenericError("Invalid Fee Amount".to_string()));
    }

    Ok(())
}

//////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
////////////// Query
///////////~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetNoisProxy{} => to_binary(&get_nois(deps)?),
    }
}

pub fn get_nois(deps: Deps) -> StdResult<Binary> {
    let nois = NOIS_PROXY.load(deps.storage)?;
    to_binary(&NoisProxyResponse {nois_proxy: nois.to_string()})
}


#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        let a = true;
        assert_eq!(a, true);
    }
}

