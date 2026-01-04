use std::io::{self, BufRead};
use uuid::Uuid;

use crate::formatters::{Format, FormatOptions, format_uuid};

pub struct FmtArgs {
    pub uuid: Option<String>,
    pub compact: bool,
    pub hex: bool,
    pub uppercase: bool,
}

pub fn run(args: FmtArgs) {
    let input = match args.uuid {
        Some(uuid) => uuid,
        None => read_from_stdin(),
    };

    let uuid = match parse_uuid(&input) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let format = if args.hex {
        Format::Hex
    } else if args.compact {
        Format::Compact
    } else {
        Format::Canonical
    };

    let options = FormatOptions {
        format,
        uppercase: args.uppercase,
    };

    println!("{}", format_uuid(&uuid, options));
}

fn read_from_stdin() -> String {
    let stdin = io::stdin();
    let mut line = String::new();
    if let Err(e) = stdin.lock().read_line(&mut line) {
        eprintln!("Error reading from stdin: {}", e);
        std::process::exit(1);
    }
    line.trim().to_string()
}

fn parse_uuid(input: &str) -> Result<Uuid, String> {
    let cleaned = input.trim();

    // Handle hex prefix
    let cleaned = cleaned.strip_prefix("0x").unwrap_or(cleaned);
    let cleaned = cleaned.strip_prefix("0X").unwrap_or(cleaned);

    Uuid::parse_str(cleaned).map_err(|e| format!("Invalid UUID '{}': {}", input, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uuid_canonical() {
        let result = parse_uuid("550e8400-e29b-41d4-a716-446655440000");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().to_string(),
            "550e8400-e29b-41d4-a716-446655440000"
        );
    }

    #[test]
    fn test_parse_uuid_compact() {
        let result = parse_uuid("550e8400e29b41d4a716446655440000");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_uuid_hex_prefix() {
        let result = parse_uuid("0x550e8400e29b41d4a716446655440000");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_uuid_hex_prefix_uppercase() {
        let result = parse_uuid("0X550E8400E29B41D4A716446655440000");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_uuid_with_whitespace() {
        let result = parse_uuid("  550e8400-e29b-41d4-a716-446655440000  ");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_uuid_invalid() {
        let result = parse_uuid("not-a-uuid");
        assert!(result.is_err());
    }
}
