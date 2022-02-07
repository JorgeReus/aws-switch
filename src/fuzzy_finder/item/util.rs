pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    if s.starts_with("aws") {
        chars.next();
        chars.next();
        chars.next();
        return format!("AWS{}", chars.as_str());
    } else if s.starts_with("sso") {
        chars.next();
        chars.next();
        chars.next();
        return format!("SSO{}", chars.as_str());
    } else {
        return match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        };
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("aws access key id"), "AWS access key id");
        assert_eq!(capitalize("aws secret access key"), "AWS secret access key");
        assert_eq!(capitalize("aws session token"), "AWS session token");
        assert_eq!(capitalize("issued time"), "Issued time");
        assert_eq!(capitalize("expires time"), "Expires time");
        assert_eq!(capitalize("sso auto populated"), "SSO auto populated");
        assert_eq!(capitalize("region"), "Region");
        assert_eq!(capitalize("sso region"), "SSO region");
    }
}
