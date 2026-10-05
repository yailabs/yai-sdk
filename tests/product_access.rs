use serde_json::json;
use yai_sdk::workflows as dto;

#[test]
fn access_sources_do_not_fabricate_commercial_entitlements() {
    let value = json!({"schema":"yai.product_access_source.v1",
        "source":"local_development", "posture":"allowed", "operation_allowed":true,
        "development_capable":true, "development_enabled":true, "profile_revision":3,
        "commercial_limits_applied":false, "commercial_auth":null, "refusal":null,
        "account_profile":{"schema":"yai.product_account_profile.v1", "posture":"not_applicable",
            "account_ref":null,"display_name":null,"email":null,"email_verification":"not_reported",
            "observed_at_unix_ms":null,"reason":"local_development_has_no_commercial_account"}});
    let observed: dto::ProductAccessSourceObservation = serde_json::from_value(value.clone()).unwrap();
    assert!(matches!(observed.source, dto::ProductAccessSource::LocalDevelopment));
    assert!(observed.commercial_auth.is_none());
    assert!(!observed.commercial_limits_applied);
    let mut leaked = value;
    leaked["account_profile"]["access_token"] = json!("not-a-real-token");
    assert!(serde_json::from_value::<dto::ProductAccessSourceObservation>(leaked).is_err());
}

#[test]
fn account_profile_absence_is_independent_of_licence_and_local_identity() {
    let value = json!({"schema":"yai.product_account_profile.v1","posture":"unavailable",
        "account_ref":"account:synthetic","display_name":null,"email":null,
        "email_verification":"not_reported","observed_at_unix_ms":null,
        "reason":"commercial_profile_contract_unavailable"});
    let profile: dto::ProductAccountProfileObservation = serde_json::from_value(value).unwrap();
    assert_eq!(profile.account_ref.as_deref(), Some("account:synthetic"));
    assert!(profile.email.is_none());
    assert!(profile.display_name.is_none());
    assert!(matches!(profile.posture, dto::AccountProfilePosture::Unavailable));
    assert!(serde_json::from_value::<dto::ProductDevelopmentAccessInput>(
        json!({"expected_revision":1,"account_ref":"account:synthetic"})).is_err());
}
