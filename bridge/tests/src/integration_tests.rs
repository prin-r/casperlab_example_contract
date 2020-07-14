use hex;
use obi::*;
use tiny_keccak::{Hasher, Keccak};

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
        keccak.finDELOYER_ACCOUNTze(&mut output);
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
pub struct MyPacket {
    pub req: Req,
    pub res: Res,
}

impl MyPacket {
    pub fn new_test_packet() -> Self {
        Self {
            req: Req {
                client_id: "front_end".into(),
                oracle_script_id: 1,
                calldata: vec![0, 0, 0, 3, 66, 84, 67, 0, 0, 0, 0, 59, 154, 202, 0],
                ans_count: 4,
                min_count: 2,
            },
            res: Res {
                client_id: "front_end".into(),
                request_id: 13565,
                ans_count: 4,
                request_time: 1592549507,
                resolve_time: 1592549511,
                resolve_status: 1,
                result: vec![0, 0, 0, 0, 0, 0, 0, 0],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use casperlabs_engine_test_support::{
        Code, Hash, SessionBuilder, TestContext, TestContextBuilder
    };
    use casperlabs_types::{
        account::AccountHash, bytesrepr::FromBytes, 
        runtime_args, CLTyped, RuntimeArgs, U512,
    };
    use hex;
    use super::*;

    const DELOYER_ACCOUNT: AccountHash = AccountHash::new([7u8; 32]);
    const BOB: AccountHash = AccountHash::new([8u8; 32]);

    const CONTRACT_NAME: &str = "pocket_storage_contract";
    const CONTRACT_HASH: &str = "pocket_storage_contract_hash";
    const RELAY_AND_VERIFY_METHOD: &str = "relay_and_verify";
    const PROOF_ARG: &str = "proof";

    #[test]
    fn should_store_relay_and_verify() {
        // Mock data.
        let mock_packet = MyPacket::new_test_packet();
        let key = hex::encode(&mock_packet.req.get_hash());
        let value = hex::decode("0000000966726f6e745f656e6400000000000000010000000f00000003425443000000003b9aca00000000000000000400000000000000020000000966726f6e745f656e6400000000000034fd0000000000000004000000005eec6083000000005eec608701000000080000000000000000").unwrap();

        // Start the context and deploy the contract.
        let mut context = TestContextBuilder::new()
            .with_account(DELOYER_ACCOUNT, U512::from(128_000_000))
            .build();
        deploy_contract(&mut context);

        // Call relay_and_verify by the account that never interacted with the contract.
        call_relay_and_verify(&mut context, value, BOB);

        // Verify the response.
        let resp_packet: MyPacket = query_contract_for_packet(&context, &key).unwrap();
        assert_eq!(resp_packet, mock_packet);
    }

    // Deploy generated wasm file.
    fn deploy_contract(context: &mut TestContext) {
        let session_code = Code::from("contract.wasm");
        let session = SessionBuilder::new(session_code, runtime_args!{})
            .with_address(DELOYER_ACCOUNT)
            .with_authorization_keys(&[DELOYER_ACCOUNT])
            .build();
        context.run(session);
    }

    // Read the packet from the blockchain using `query` method.
    fn query_contract_for_packet(context: &TestContext, name: &str) -> Option<MyPacket> {
        match context.query(DELOYER_ACCOUNT, &[CONTRACT_NAME, name]) {
            Err(e) => None,
            Ok(maybe_value) => {
                let proof: Vec<u8> = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(MyPacket::try_from_slice(&proof).unwrap())
            }
        }
    }

    // Make a call to the smart contract.
    fn call_relay_and_verify(context: &mut TestContext, value: Vec<u8>, sender: AccountHash) {
        let hash = contract_hash(&context);
        let code = Code::Hash(hash, RELAY_AND_VERIFY_METHOD.to_string());
        let args = runtime_args!{
            PROOF_ARG => value
        };
        let session = SessionBuilder::new(code, args)
            .with_address(sender)
            .with_authorization_keys(&[sender])
            .build();
        context.run(session);
    }

    // Query the blockchain to retrieve contract's hash.
    fn contract_hash(context: &TestContext) -> Hash {
        context
            .query(DELOYER_ACCOUNT, &[CONTRACT_HASH])
            .unwrap_or_else(|_| panic!("{} contract not found", CONTRACT_NAME))
            .into_t()
            .unwrap_or_else(|_| panic!("{} has wrong type", CONTRACT_NAME))
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
