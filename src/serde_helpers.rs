/// Serde helpers for Proxmox API compatibility.
///
/// The Proxmox VE API uses `0`/`1` integers for boolean fields. These helpers
/// allow Rust code to use `Option<bool>` while maintaining wire compatibility.
/// Serializes `Option<bool>` as `0`/`1` integers and deserializes from both
/// integer (`0`/`1`) and native boolean (`true`/`false`) representations.
pub(crate) mod option_bool_as_int {
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// Serializes `Option<bool>` as an integer: `true` → `1`, `false` → `0`.
    ///
    /// When paired with `skip_serializing_if = "Option::is_none"`, the `None`
    /// case is never reached — but we handle it defensively.
    #[allow(dead_code)]
    pub fn serialize<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(true) => serializer.serialize_i64(1),
            Some(false) => serializer.serialize_i64(0),
            None => serializer.serialize_none(),
        }
    }

    /// Deserializes `Option<bool>` from integer (`0`/`1`) or boolean (`true`/`false`).
    #[allow(dead_code)]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum IntOrBool {
            Int(i64),
            Bool(bool),
        }

        match Option::<IntOrBool>::deserialize(deserializer)? {
            Some(IntOrBool::Int(0)) => Ok(Some(false)),
            Some(IntOrBool::Int(_)) => Ok(Some(true)),
            Some(IntOrBool::Bool(b)) => Ok(Some(b)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "super::option_bool_as_int"
        )]
        enabled: Option<bool>,
    }

    #[test]
    fn serialize_true_as_1() {
        let s = TestStruct { enabled: Some(true) };
        let json = serde_json::to_value(&s).unwrap();
        assert_eq!(json["enabled"], 1);
    }

    #[test]
    fn serialize_false_as_0() {
        let s = TestStruct { enabled: Some(false) };
        let json = serde_json::to_value(&s).unwrap();
        assert_eq!(json["enabled"], 0);
    }

    #[test]
    fn serialize_none_skipped() {
        let s = TestStruct { enabled: None };
        let json = serde_json::to_value(&s).unwrap();
        assert!(!json.as_object().unwrap().contains_key("enabled"));
    }

    #[test]
    fn deserialize_int_1_as_true() {
        let s: TestStruct = serde_json::from_str(r#"{"enabled": 1}"#).unwrap();
        assert_eq!(s.enabled, Some(true));
    }

    #[test]
    fn deserialize_int_0_as_false() {
        let s: TestStruct = serde_json::from_str(r#"{"enabled": 0}"#).unwrap();
        assert_eq!(s.enabled, Some(false));
    }

    #[test]
    fn deserialize_bool_true() {
        let s: TestStruct = serde_json::from_str(r#"{"enabled": true}"#).unwrap();
        assert_eq!(s.enabled, Some(true));
    }

    #[test]
    fn deserialize_bool_false() {
        let s: TestStruct = serde_json::from_str(r#"{"enabled": false}"#).unwrap();
        assert_eq!(s.enabled, Some(false));
    }

    #[test]
    fn deserialize_absent_as_none() {
        let s: TestStruct = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(s.enabled, None);
    }

    #[test]
    fn roundtrip_preserves_value() {
        let original = TestStruct { enabled: Some(true) };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: TestStruct = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
