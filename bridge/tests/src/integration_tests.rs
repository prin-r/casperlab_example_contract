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
    use casperlabs_engine_test_support::{Code, Error, SessionBuilder, TestContextBuilder, Value};
    use casperlabs_types::{account::PublicKey, U512};
    use hex;

    const MY_ACCOUNT: PublicKey = PublicKey::ed25519_from([7u8; 32]);

    #[test]
    fn should_store_relay_and_verify() {
        let mut context = TestContextBuilder::new()
            .with_account(MY_ACCOUNT, U512::from(128_000_000))
            .build();

        let mock_packet = MyPacket::new_test_packet();
        let key = hex::encode(&mock_packet.req.get_hash());
        let value = hex::decode("0000000966726f6e745f656e6400000000000000010000000f00000003425443000000003b9aca0000000000000000040000000000000002").unwrap();

        println!("{:?}", mock_packet.req.get_hash());
        println!("{:?}", hex::encode(mock_packet.req.try_to_vec().unwrap()));

        // assert_eq!(
        //     hex::encode(&mock_packet.req.get_hash()),
        //     String::from("0xaa")
        // );

        // The test framework checks for compiled Wasm files in '<current working dir>/wasm'.  Paths
        // relative to the current working dir (e.g. 'wasm/contract.wasm') can also be used, as can
        // absolute paths.
        let session_code = Code::from("contract.wasm");
        let session_args = ("relay_and_verify", value.clone());
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(MY_ACCOUNT)
            .with_authorization_keys(&[MY_ACCOUNT])
            .build();

        let result_of_query: Result<Value, Error> = context.run(session).query(MY_ACCOUNT, &[&key]);

        let returned_value = result_of_query.expect("should be a value");

        let expected_value =
            Value::from_t(mock_packet.try_to_vec().unwrap()).expect("should construct Value");
        assert_eq!(expected_value, returned_value);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
