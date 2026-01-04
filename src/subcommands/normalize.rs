use uuid::Uuid;

pub struct NormalizeArgs {
    pub uuid: String,
}

pub fn run(args: NormalizeArgs) {
    let input = args.uuid.trim();

    // Remove hyphens and any 0x prefix
    let cleaned = input.strip_prefix("0x").unwrap_or(input);
    let cleaned = cleaned.strip_prefix("0X").unwrap_or(cleaned);
    let hex_str: String = cleaned.chars().filter(|c| *c != '-').collect();

    if hex_str.len() != 32 {
        eprintln!(
            "Error: Invalid UUID length. Expected 32 hex characters, got {}",
            hex_str.len()
        );
        std::process::exit(1);
    }

    let bytes = match hex_to_bytes(&hex_str) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let normalized = normalize_guid_bytes(bytes);
    let uuid = Uuid::from_bytes(normalized);

    println!("{}", uuid.simple().to_string().to_uppercase());
}

/// Converts .NET System.Guid mixed byte order to RFC4122 format.
///
/// .NET System.Guid has mixed endianness:
/// - Bytes 0-3 (Data1): Little-endian (needs reversal)
/// - Bytes 4-5 (Data2): Little-endian (needs reversal)
/// - Bytes 6-7 (Data3): Little-endian (needs reversal)
/// - Bytes 8-15: Big-endian (stays as-is)
fn normalize_guid_bytes(bytes: [u8; 16]) -> [u8; 16] {
    let mut normalized = [0u8; 16];

    // Reverse Data1 (bytes 0-3)
    normalized[0] = bytes[3];
    normalized[1] = bytes[2];
    normalized[2] = bytes[1];
    normalized[3] = bytes[0];

    // Reverse Data2 (bytes 4-5)
    normalized[4] = bytes[5];
    normalized[5] = bytes[4];

    // Reverse Data3 (bytes 6-7)
    normalized[6] = bytes[7];
    normalized[7] = bytes[6];

    // Keep bytes 8-15 as-is
    normalized[8..16].copy_from_slice(&bytes[8..16]);

    normalized
}

fn hex_to_bytes(hex: &str) -> Result<[u8; 16], String> {
    if hex.len() != 32 {
        return Err(format!("Expected 32 hex characters, got {}", hex.len()));
    }

    let mut bytes = [0u8; 16];
    for i in 0..16 {
        let byte_str = &hex[i * 2..i * 2 + 2];
        bytes[i] = u8::from_str_radix(byte_str, 16)
            .map_err(|_| format!("Invalid hex character in '{}'", byte_str))?;
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_guid_bytes() {
        // Example from docs: D004A78FD44C1B4E8213324AE10814DC -> 8FA704D04CD44E1B8213324AE10814DC
        let input = hex_to_bytes("D004A78FD44C1B4E8213324AE10814DC").unwrap();
        let normalized = normalize_guid_bytes(input);
        let result = Uuid::from_bytes(normalized);

        assert_eq!(
            result.simple().to_string().to_uppercase(),
            "8FA704D04CD44E1B8213324AE10814DC"
        );
    }

    #[test]
    fn test_hex_to_bytes_valid() {
        let result = hex_to_bytes("550e8400e29b41d4a716446655440000");
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert_eq!(bytes[0], 0x55);
        assert_eq!(bytes[1], 0x0e);
    }

    #[test]
    fn test_hex_to_bytes_invalid_length() {
        let result = hex_to_bytes("550e8400");
        assert!(result.is_err());
    }

    #[test]
    fn test_hex_to_bytes_invalid_chars() {
        let result = hex_to_bytes("ZZZZ8400e29b41d4a716446655440000");
        assert!(result.is_err());
    }

    #[test]
    fn test_normalize_preserves_last_8_bytes() {
        let input = hex_to_bytes("000000000000000001020304050607FF").unwrap();
        let normalized = normalize_guid_bytes(input);

        // Last 8 bytes should be unchanged
        assert_eq!(
            normalized[8..16],
            [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0xFF]
        );
    }

    #[test]
    fn test_normalize_reverses_first_4_bytes() {
        let input = hex_to_bytes("01020304000000000000000000000000").unwrap();
        let normalized = normalize_guid_bytes(input);

        // First 4 bytes should be reversed
        assert_eq!(normalized[0..4], [0x04, 0x03, 0x02, 0x01]);
    }

    #[test]
    fn test_normalize_reverses_data2() {
        let input = hex_to_bytes("00000000AABB00000000000000000000").unwrap();
        let normalized = normalize_guid_bytes(input);

        // Bytes 4-5 should be reversed
        assert_eq!(normalized[4..6], [0xBB, 0xAA]);
    }

    #[test]
    fn test_normalize_reverses_data3() {
        let input = hex_to_bytes("000000000000CCDD0000000000000000").unwrap();
        let normalized = normalize_guid_bytes(input);

        // Bytes 6-7 should be reversed
        assert_eq!(normalized[6..8], [0xDD, 0xCC]);
    }
}
