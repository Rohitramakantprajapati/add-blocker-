//! VoidBlock placeholder root module retained for compatibility with the
//! existing workspace snapshot.

pub const VOIDBLOCK_VERSION: &str = "0.1.0";

#[cfg(test)]
mod tests {
    use super::VOIDBLOCK_VERSION;

    #[test]
    fn version_is_set() {
        assert_eq!(VOIDBLOCK_VERSION, "0.1.0");
    }
}
