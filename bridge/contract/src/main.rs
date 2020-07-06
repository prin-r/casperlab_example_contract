#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]
mod api;
mod error;
use crate::api::Api;
use crate::error::Error;
use casperlabs_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casperlabs_types::{ApiError, ContractRef, Key, URef, U512};
use obi::{OBIDecode, OBIEncode};
use tiny_keccak::{Hasher, Keccak};

pub const INIT_FLAG_KEY: [u8; 32] = [1u8; 32];
const KEY: &str = "special_value";

#[derive(Clone, Debug, PartialEq, OBIDecode, OBIEncode)]
pub struct Req {
    pub client_id: String,
    pub oracle_script_id: u64,
    pub calldata: Vec<u8>,
    pub ans_count: u64,
    pub min_count: u64,
}

impl Req {
    pub fn get_hash(input: &[u8]) -> [u8; 32] {
        let keccak = Keccak::v256();
        let mut output = [0u8; 32];
        keccak.finalize(&mut output);
        output
    }
}

#[derive(Clone, Debug, PartialEq, OBIDecode, OBIEncode)]
pub struct Res {
    pub client_id: String,
    pub request_id: u64,
    pub ans_count: u64,
    pub request_time: u64,
    pub resolve_time: u64,
    pub resolve_status: u8,
    pub result: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, OBIDecode, OBIEncode)]
pub struct BandPacket {
    pub req: Req,
    pub res: Res,
}

fn is_not_initialized() -> bool {
    let flag: Option<i32> = storage::read_local(&INIT_FLAG_KEY).unwrap_or_revert();
    flag.is_none()
}

fn mark_as_initialized() {
    storage::write_local(INIT_FLAG_KEY, 1);
}

fn entry_point() {
    match Api::from_args() {
        Api::RELAY_AND_VERIFY(proof) => {
            let bp = BandPacket::try_from_slice(&proof).unwrap();

            let value_ref: URef = storage::new_uref(proof);
            let value_key: Key = value_ref.into();
            runtime::put_key(KEY, value_key);
        }
        // Api::Approve(spender, amount) => token.approve(&runtime::get_caller(), &spender, amount),
        // Api::BalanceOf(address) => {
        //     runtime::ret(CLValue::from_t(token.balance_of(&address)).unwrap_or_revert())
        // }
        _ => runtime::revert(Error::UnknownBridgeCallCommand),
    }
}

// All session code must have a `call` entrypoint.
#[no_mangle]
pub extern "C" fn call() {
    if is_not_initialized() {
        mark_as_initialized();
    } else {
        entry_point();
    }
}
