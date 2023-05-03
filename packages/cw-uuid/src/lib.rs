use uuid::{Builder, Bytes};

pub fn get_uuid(
    block_height: u64,
    entropy: u32 
) -> String {
    let mut bytes: Bytes = [0; 16];

    bytes[..4].copy_from_slice(&entropy.to_le_bytes());

    bytes[4..12].copy_from_slice(&block_height.to_le_bytes());

    Builder::from_random_bytes(bytes).into_uuid().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_uuid() {
        let block_height: u64 = 222352300;
        let entropy: u32 = 7;

        let my_uuid = get_uuid(block_height, entropy);
        assert_eq!(my_uuid, String::from("07000000-acd3-400d-8000-000000000000"));
    }
}
