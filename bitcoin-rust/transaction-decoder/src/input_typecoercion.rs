use std::fmt;
use std::io::Read;

use crate::{decode::read_compact_size, transaction::read_u32};

#[allow(dead_code)]
#[derive(Debug)]
struct Input {
    txid: [u8; 32],
    output_index: u32,
    script: Vec<u8>,
    sequence: u32,
}

// impl fmt::Debug for Input {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Input")
//             .field("txid", &self.txid)
//             .field("output_index", &self.output_index)
//             .field("script", &self.script)
//             .field("sequence", &self.sequence)
//             .finish()
//     }
// }

pub fn module_main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";

    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_u32(&mut bytes_slice);
    let input_length = read_compact_size(&mut bytes_slice).unwrap();
    let mut inputs = vec![];

    for _ in 0..input_length {
        let txid = read_txid(&mut bytes_slice);
        let output_index = read_u32(&mut bytes_slice);
        let script = read_script(&mut bytes_slice);
        let sequence = read_u32(&mut bytes_slice);

        let input = Input {
            txid,
            output_index,
            script,
            sequence,
        };
        inputs.push(input);
    }

    println!("Version: {}", version);
    println!("Inputs: {:?}", inputs);
}

fn read_txid(transaction_bytes: &mut &[u8]) -> [u8; 32] {
    let mut buffer = [0_u8; 32];
    transaction_bytes.read(&mut buffer).unwrap();
    buffer.reverse();
    buffer
}

fn read_script(transaction_bytes: &mut &[u8]) -> Vec<u8> {
    let script_size = read_compact_size(transaction_bytes).unwrap() as usize;
    let mut buffer = vec![0_u8; script_size];
    transaction_bytes.read(&mut buffer).unwrap();
    buffer
}
