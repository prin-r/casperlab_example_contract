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
    use super::*;
    use casperlabs_engine_test_support::{
        Code, Hash, SessionBuilder, TestContext, TestContextBuilder
    };
    use casperlabs_types::{
        account::AccountHash, bytesrepr::FromBytes, 
        runtime_args, CLTyped, RuntimeArgs, U512,
    };

    use hex;

    const MY_ACCOUNT: AccountHash = AccountHash::new([7u8; 32]);

    const CONTRACT_NAME: &str = "pocket_storage_contract";
    const RELAY_AND_VERIFY_METHOD: &str = "relay_and_verify";
    const PROOF_ARG: &str = "proof";

    #[test]
    fn should_store_relay_and_verify() {
        let mut context = TestContextBuilder::new()
            .with_account(MY_ACCOUNT, U512::from(128_000_000))
            .build();

        let mock_packet = MyPacket::new_test_packet();
        let key = hex::encode(&mock_packet.req.get_hash());
        let value = hex::decode("0000000966726f6e745f656e6400000000000000010000000f00000003425443000000003b9aca00000000000000000400000000000000020000000966726f6e745f656e6400000000000034fd0000000000000004000000005eec6083000000005eec608701000000080000000000000000").unwrap();

        // println!("{:?}", mock_packet.req.get_hash());
        // println!("{:?}", hex::encode(mock_packet.req.try_to_vec().unwrap()));


        deploy_contract(&mut context);
        call_relay_and_verify(&mut context, value, MY_ACCOUNT);
        // let value: Vec<u8> = query_contract(&context, "bbb").unwrap();
        let value = context.get_account(MY_ACCOUNT);
        println!("{:?}", value);


        // assert_eq!(
        //     hex::encode(&mock_packet.try_to_vec().unwrap()),
        //     String::from("0xaa")
        // );

        // The test framework checks for compiled Wasm files in '<current working dir>/wasm'.  Paths
        // relative to the current working dir (e.g. 'wasm/contract.wasm') can also be used, as can
        // absolute paths.

        // let value = context.query(MY_ACCOUNT, &[CONTRACT_NAME, "ccc"]);
        // println!("{:?}", value);
        // assert_eq!(1, 2);

        // println!("{:?}", hash);
        // assert_eq!(1, 2);
        // let result_of_query: Result<Value, Error> = context.run(session).query(MY_ACCOUNT, &[&key]);

        // let returned_value = result_of_query.expect("should be a value");

        // let expected_value =
        //     Value::from_t(mock_packet.try_to_vec().unwrap()).expect("should construct Value");
        // assert_eq!(expected_value, returned_value);
    }

    fn deploy_contract(context: &mut TestContext) {
        let session_code = Code::from("contract.wasm");
        let session = SessionBuilder::new(session_code, runtime_args!{})
            .with_address(MY_ACCOUNT)
            .with_authorization_keys(&[MY_ACCOUNT])
            .build();
        context.run(session);
    }

    fn query_contract<T: CLTyped + FromBytes>(context: &TestContext, name: &str) -> Option<T> {
        match context.query(
            MY_ACCOUNT,
            &[CONTRACT_NAME, &name.to_string()],
        ) {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

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

    fn contract_hash(context: &TestContext) -> Hash {
        context
            .query(MY_ACCOUNT, &[CONTRACT_NAME])
            .unwrap_or_else(|_| panic!("{} contract not found", CONTRACT_NAME))
            .into_t()
            .unwrap_or_else(|_| panic!("{} has wrong type", CONTRACT_NAME))
    }


}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
