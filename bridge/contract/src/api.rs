use casperlabs_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casperlabs_types::{account::PublicKey, bytesrepr::FromBytes, CLTyped, ContractRef, U512};

use crate::error::Error;

pub const RELAY_AND_VERIFY: &str = "relay_and_verify";

pub enum Api {
    RELAY_AND_VERIFY(Vec<u8>),
}

fn get_arg<T: CLTyped + FromBytes>(i: u32) -> T {
    runtime::get_arg(i)
        .unwrap_or_revert_with(Error::missing_argument(i))
        .unwrap_or_revert_with(Error::invalid_argument(i))
}

impl Api {
    pub fn from_args() -> Api {
        let method_name: String = get_arg(0);
        match method_name.as_str() {
            RELAY_AND_VERIFY => {
                let proof: Vec<u8> = get_arg(1);
                Api::RELAY_AND_VERIFY(proof)
            }
            _ => runtime::revert(Error::UnknownApiCommand),
        }
    }
}
