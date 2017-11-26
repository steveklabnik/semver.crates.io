extern crate semver;

use semver::Version;
use semver::VersionReq;

#[no_mangle]
pub fn is_match(v: &str, r: &str) -> bool {
    let v = match Version::parse(v) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let r = match VersionReq::parse(r) {
        Ok(r) => r,
        Err(_) => return false,
    };

    r.matches(&v)
}

#[cfg(test)]
mod tests {
    use super::is_match;

    #[test]
    fn matches() {
        assert!(is_match("1.0.0", ">= 1.0.0"));
    }

    #[test]
    fn does_not_match() {
        assert!(!is_match("0.1.0", ">= 1.0.0"));
    }
}

