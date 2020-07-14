#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]

mod error;

use hex;
use obi::{OBIDecode, OBIEncode};
use tiny_keccak::{Hasher, Keccak};

use casperlabs_contract::{
    contract_api::{account, runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casperlabs_types::{
    account::AccountHash,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    runtime_args, CLType, CLTyped, Group, Key, Parameter, RuntimeArgs, URef, U512,
};

use crate::error::Error;

pub const CONTRACT_NAME: &str = "pocket_storage_contract";
pub const CONTRACT_HASH: &str = "pocket_storage_contract_hash";
pub const RELAY_AND_VERIFY_METHOD: &str = "relay_and_verify";
pub const PROOF_ARG: &str = "proof";

#[derive(Clone, Debug, PartialEq, OBIDecode, OBIEncode)]
pub struct MyPacket {
    pub req: Req,
    pub res: Res,
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
pub struct Req {
    pub client_id: String,
    pub oracle_script_id: u64,
    pub calldata: Vec<u8>,
    pub ans_count: u64,
    pub min_count: u64,
}

impl Req {
    pub fn get_hash(&self) -> [u8; 32] {
        let mut keccak = Keccak::v256();
        let mut output = [0u8; 32];
        keccak.update(&(self.try_to_vec().unwrap()));
        keccak.finalize(&mut output);
        output
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        RELAY_AND_VERIFY_METHOD.to_string(),
        vec![Parameter::new(PROOF_ARG, Vec::<u8>::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    let (contract, _version) = storage::new_contract(
        entry_points, None, None, None
    );

    // Save a contract 
    runtime::put_key(CONTRACT_NAME, contract.into());

    // Save a hash
    let contract_hash: Key = storage::new_uref(contract).into();
    runtime::put_key(CONTRACT_HASH, contract_hash);

}

#[no_mangle]
pub extern "C" fn relay_and_verify() {
    // let value = storage::new_uref(123).into();
    // runtime::put_key("bbb", value);

    let proof: Vec<u8> = runtime::get_named_arg(PROOF_ARG);
    match MyPacket::try_from_slice(&proof) {
        Ok(bp) => {
            let value = storage::new_uref(proof).into();
            runtime::put_key(&hex::encode(&bp.req.get_hash()), value);
        }
        Err(_) => runtime::revert(Error::FailToDecodeProof),
    }
}
