use crate::{
    error::ContractError,
    msg::{DataResponse, ExecuteMsg, InstantiateMsg, QueryMsg, SaveDataMsg},
    state::{StoredData, STORED_DATA},
    utils::decode_hex,
};
use babylon_bindings::{BabylonQuerier, BabylonQuery};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint64,
};
use cw2::set_contract_version;
use sha2::{Digest, Sha256};

// Version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<BabylonQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<BabylonQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SaveData { save_data } => execute_save_data(deps, _env, _info, save_data),
    }
}

pub fn execute_save_data(
    deps: DepsMut<BabylonQuery>,
    _env: Env,
    _info: MessageInfo,
    save_data: SaveDataMsg,
) -> Result<Response, ContractError> {
    // rollup details
    let da_height = save_data.da_height;
    let data = save_data.data;
    let data_bytes = decode_hex(&data)?;
    let hash = Sha256::digest(data_bytes);
    let hash_string = hex::encode(hash);

    if STORED_DATA.has(deps.storage, &da_height) {
        return Err(ContractError::DataAlreadyExists {});
    }

    // Add BTC timestamp info
    let bq = BabylonQuerier::new(&deps.querier);
    let btc_tip = bq.btc_tip()?;
    let btc_height = Uint64::from(btc_tip.height);
    let btc_timestamp = Uint64::from(btc_tip.header.time as u64);
    let saved_at_btc_epoch = bq.current_epoch()?;
    let stored_data = StoredData {
        data_hash: hash_string.clone(),
        data: data.clone(),
        btc_height,
        btc_timestamp,
        saved_at_btc_epoch,
        da_height: da_height.clone(),
    };

    STORED_DATA.save(deps.storage, &da_height, &stored_data)?;

    Ok(Response::default()
        .add_attribute("action", "save_data")
        .add_attribute("da_height", da_height.to_string())
        .add_attribute("data_hash", hash_string)
        .add_attribute("data", data)
        .add_attribute("btc_height", btc_height)
        .add_attribute("btc_timestamp", btc_timestamp)
        .add_attribute("saved_at_btc_epoch", saved_at_btc_epoch))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<BabylonQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryData { query_data } => {
            let data = STORED_DATA.load(deps.storage, &query_data.da_height)?;
            let bq = BabylonQuerier::new(&deps.querier);
            let latest_finalized_epoch_info_res = bq.latest_finalized_epoch_info();

            // Realistically there can be only one error here i.e there is no finalized epoch
            let latest_finalized_epoch = match latest_finalized_epoch_info_res {
                Ok(epoch_info) => Uint64::from(epoch_info.epoch_number),
                Err(_) => Uint64::zero(),
            };

            to_json_binary(&DataResponse {
                finalized: latest_finalized_epoch >= data.saved_at_btc_epoch,
                latest_finalized_epoch,
                data,
            })
        }
    }
}
