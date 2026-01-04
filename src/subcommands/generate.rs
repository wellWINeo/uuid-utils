use crate::formatters::{Format, FormatOptions, format_uuid};
use crate::generators::{Version, generate};

pub struct GenArgs {
    pub version: String,
    pub uppercase: bool,
    pub compact: bool,
    pub hex: bool,
}

pub fn run(args: GenArgs) {
    let version = match Version::from_str(&args.version) {
        Some(v) => v,
        None => {
            eprintln!(
                "Error: Unknown UUID version '{}'. Supported: v4, v7",
                args.version
            );
            std::process::exit(1);
        }
    };

    let uuid = generate(version);

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
    fn test_gen_args_default() {
        let args = GenArgs {
            version: "v4".to_string(),
            uppercase: false,
            compact: false,
            hex: false,
        };
        assert_eq!(args.version, "v4");
        assert!(!args.uppercase);
    }
}
