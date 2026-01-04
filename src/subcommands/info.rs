use uuid::Uuid;

pub struct InfoArgs {
    pub uuid: String,
}

pub fn run(args: InfoArgs) {
    let uuid = match parse_uuid(&args.uuid) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    print_uuid_info(&uuid);
}

fn parse_uuid(input: &str) -> Result<Uuid, String> {
    let cleaned = input.trim();
    let cleaned = cleaned.strip_prefix("0x").unwrap_or(cleaned);
    let cleaned = cleaned.strip_prefix("0X").unwrap_or(cleaned);

    Uuid::parse_str(cleaned).map_err(|e| format!("Invalid UUID '{}': {}", input, e))
}

fn print_uuid_info(uuid: &Uuid) {
    let version = uuid.get_version_num();
    println!("version: {}", version);

    match version {
        1 => print_v1_info(uuid),
        4 => print_v4_info(),
        7 => print_v7_info(uuid),
        _ => println!("variant: {}", get_variant_name(uuid)),
    }
}

fn print_v1_info(uuid: &Uuid) {
    println!("variant: RFC4122");

    if let Some(ts) = uuid.get_timestamp() {
        let (seconds, nanos) = ts.to_unix();
        println!("timestamp: {}.{:09} (Unix epoch)", seconds, nanos);
    }
}

fn print_v4_info() {
    println!("variant: RFC4122");
    println!("type: random");
}

fn print_v7_info(uuid: &Uuid) {
    println!("variant: RFC4122");

    if let Some(ts) = uuid.get_timestamp() {
        let (seconds, nanos) = ts.to_unix();
        println!("timestamp: {}.{:09} (Unix epoch)", seconds, nanos);

        // Convert to human-readable format
        let millis = seconds * 1000 + (nanos / 1_000_000) as u64;
        println!("timestamp_ms: {}", millis);
    }
}

fn get_variant_name(uuid: &Uuid) -> &'static str {
    match uuid.get_variant() {
        uuid::Variant::NCS => "NCS",
        uuid::Variant::RFC4122 => "RFC4122",
        uuid::Variant::Microsoft => "Microsoft",
        uuid::Variant::Future => "Future",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uuid_valid() {
        let result = parse_uuid("550e8400-e29b-41d4-a716-446655440000");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_uuid_with_hex_prefix() {
        let result = parse_uuid("0x550e8400e29b41d4a716446655440000");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_uuid_invalid() {
        let result = parse_uuid("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_variant_name() {
        let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        assert_eq!(get_variant_name(&uuid), "RFC4122");
    }

    #[test]
    fn test_v4_uuid_version() {
        let uuid = Uuid::new_v4();
        assert_eq!(uuid.get_version_num(), 4);
    }

    #[test]
    fn test_v7_uuid_version() {
        let uuid = Uuid::now_v7();
        assert_eq!(uuid.get_version_num(), 7);
    }
}
