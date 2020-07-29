import casperlabs_client
from casperlabs_client import abi
from casperlabs_client import consensus_pb2 as consensus, state_pb2 as state

Arg = consensus.Deploy.Arg
Instance = state.CLValueInstance
Value = Instance.Value
Type = state.CLType


def to_abi_bytes_list(name, bytes):
    bytes_type = Type(simple_type=Type.Simple.U8)
    values = [Value(u8=byte) for byte in bytes]
    return Arg(
        name=name,
        value=Instance(
            cl_type=Type(list_type=Type.List(inner=bytes_type)),
            value=Value(list_value=Instance.List(values=values)),
        )
    )


client = casperlabs_client.CasperLabsClient("deploy.casperlabs.io", 40401)
my_pocket = bytes.fromhex('0000000966726f6e745f656e6400000000000000010000000f00000003425443000000003b9aca00000000000000000400000000000000020000000966726f6e745f656e6400000000000034fd0000000000000004000000005eec6083000000005eec608701000000080000000000000000')

args = [
    abi.ABI.string_value('method', 'relay_and_verify'),
    to_abi_bytes_list('bytes', my_pocket)
]

deploy_hash = client.deploy(
    from_addr='3cf191a1f42d9e71dcaec5d21991500818c93859a6e0c61673935c5db325620c',
    private_key='counter.private.key',
    session='contract/target/wasm32-unknown-unknown/release/contract.wasm',
    session_args=abi.ABI.args(args),
    gas_price=10,
    payment_amount=2000000,
)
print(f'Contract deploy under hash: {deploy_hash}')

# resp = client.wait_for_deploy_processed(deploy_hash)
# print(resp)
