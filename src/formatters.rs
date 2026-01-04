use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default)]
pub enum Format {
    #[default]
    Canonical,
    Compact,
    Hex,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FormatOptions {
    pub format: Format,
    pub uppercase: bool,
}

pub fn format_uuid(uuid: &Uuid, options: FormatOptions) -> String {
    let formatted = match options.format {
        Format::Canonical => canonical(uuid),
        Format::Compact => compact(uuid),
        Format::Hex => hex(uuid),
    };

    if options.uppercase {
        formatted.to_uppercase()
    } else {
        formatted
    }
}

pub fn canonical(uuid: &Uuid) -> String {
    uuid.hyphenated().to_string()
}

pub fn compact(uuid: &Uuid) -> String {
    uuid.simple().to_string()
}

pub fn hex(uuid: &Uuid) -> String {
    format!("0x{}", uuid.simple())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_uuid() -> Uuid {
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap()
    }

    #[test]
    fn test_canonical() {
        assert_eq!(
            canonical(&test_uuid()),
            "550e8400-e29b-41d4-a716-446655440000"
        );
    }

    #[test]
    fn test_compact() {
        assert_eq!(compact(&test_uuid()), "550e8400e29b41d4a716446655440000");
    }

    #[test]
    fn test_hex() {
        assert_eq!(hex(&test_uuid()), "0x550e8400e29b41d4a716446655440000");
    }

    #[test]
    fn test_format_uuid_canonical() {
        let options = FormatOptions {
            format: Format::Canonical,
            uppercase: false,
        };
        assert_eq!(
            format_uuid(&test_uuid(), options),
            "550e8400-e29b-41d4-a716-446655440000"
        );
    }

    #[test]
    fn test_format_uuid_compact_uppercase() {
        let options = FormatOptions {
            format: Format::Compact,
            uppercase: true,
        };
        assert_eq!(
            format_uuid(&test_uuid(), options),
            "550E8400E29B41D4A716446655440000"
        );
    }

    #[test]
    fn test_format_uuid_hex_uppercase() {
        let options = FormatOptions {
            format: Format::Hex,
            uppercase: true,
        };
        assert_eq!(
            format_uuid(&test_uuid(), options),
            "0X550E8400E29B41D4A716446655440000"
        );
    }
}
