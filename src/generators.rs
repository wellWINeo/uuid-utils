use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default)]
pub enum Version {
    #[default]
    V4,
    V7,
}

impl Version {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "v4" | "4" => Some(Self::V4),
            "v7" | "7" => Some(Self::V7),
            _ => None,
        }
    }
}

pub fn generate(version: Version) -> Uuid {
    match version {
        Version::V4 => generate_v4(),
        Version::V7 => generate_v7(),
    }
}

pub fn generate_v4() -> Uuid {
    Uuid::new_v4()
}

pub fn generate_v7() -> Uuid {
    Uuid::now_v7()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_v4() {
        let uuid = generate_v4();
        assert_eq!(uuid.get_version_num(), 4);
    }

    #[test]
    fn test_generate_v7() {
        let uuid = generate_v7();
        assert_eq!(uuid.get_version_num(), 7);
    }

    #[test]
    fn test_generate_with_version() {
        let uuid_v4 = generate(Version::V4);
        assert_eq!(uuid_v4.get_version_num(), 4);

        let uuid_v7 = generate(Version::V7);
        assert_eq!(uuid_v7.get_version_num(), 7);
    }

    #[test]
    fn test_version_from_str() {
        assert!(matches!(Version::from_str("v4"), Some(Version::V4)));
        assert!(matches!(Version::from_str("V4"), Some(Version::V4)));
        assert!(matches!(Version::from_str("4"), Some(Version::V4)));
        assert!(matches!(Version::from_str("v7"), Some(Version::V7)));
        assert!(matches!(Version::from_str("V7"), Some(Version::V7)));
        assert!(matches!(Version::from_str("7"), Some(Version::V7)));
        assert!(Version::from_str("v5").is_none());
        assert!(Version::from_str("invalid").is_none());
    }
}
