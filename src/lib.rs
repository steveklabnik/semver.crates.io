extern crate semver;

use semver::Version;
use semver::VersionReq;

fn is_match(v: &Version, r: &VersionReq) -> bool {
    r.matches(v)
}

#[cfg(test)]
mod tests {
    use semver::Version;
    use semver::VersionReq;
    use super::is_match;

    #[test]
    fn matches() {
        let r = VersionReq::parse(">= 1.0.0").unwrap();
        let v = Version::parse("1.0.0").unwrap();

        assert!(is_match(&v, &r));
    }

    #[test]
    fn does_not_match() {
        let r = VersionReq::parse(">= 1.0.0").unwrap();
        let v = Version::parse("0.1.0").unwrap();

        assert!(!is_match(&v, &r));
    }
}

