use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_pas_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    if v.get("resourceType").and_then(|x| x.as_str()) != Some("Claim") {
        r.push(ValidationIssue::error("resourceType", "PAS uses Claim", "pas-claim"));
    }
    if v.get("use").and_then(|x| x.as_str()) != Some("preauthorization") {
        r.push(ValidationIssue::error("use", "PAS Claim.use must be preauthorization", "pas-use"));
    }
    Ok(r)
}
