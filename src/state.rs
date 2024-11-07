use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;
use cw_storage_plus::Map;

#[cw_serde]
pub struct StoredData {
    pub data_hash: String,
    pub data: String,
    pub btc_height: Uint64,
    pub btc_timestamp: Uint64,
    pub saved_at_btc_epoch: Uint64,
    pub da_height: u64,
}

pub const STORED_DATA: Map<&u64, StoredData> = Map::new("stored_data"); // da_height -> storeData
