use crate::formatters::{Format, FormatOptions, format_uuid};
use uuid::Uuid;

pub struct NilArgs {
    pub uppercase: bool,
    pub compact: bool,
    pub hex: bool,
}

pub fn run(args: NilArgs) {
    let uuid = Uuid::nil();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nil_args_default() {
        let args = NilArgs {
            uppercase: false,
            compact: false,
            hex: false,
        };
        assert!(!args.uppercase);
        assert!(!args.compact);
        assert!(!args.hex);
    }
}
