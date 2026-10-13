
use serde_json::Value;
use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_eligibility_file(text: &str) -> ValidationReport {
    let t = text.trim();
    if t.starts_with('{') {
        return validate_eligibility_json(t).unwrap_or_else(|e| {
            let mut r = ValidationReport::ok();
            r.push(ValidationIssue::error("json", e, "json-parse"));
            r
        });
    }
    validate_271_sketch(t)
}

fn validate_eligibility_json(json: &str) -> Result<ValidationReport, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("JSON: {e}"))?;
    let mut r = ValidationReport::ok();
    let rt = v.get("resourceType").and_then(|x| x.as_str());
    if rt != Some("CoverageEligibilityRequest") && rt != Some("CoverageEligibilityResponse") {
        r.push(ValidationIssue::error("resourceType", "expected CoverageEligibilityRequest or Response", "elig-resource"));
    }
    if v.get("status").and_then(|x| x.as_str()).is_none_or(|s| s.is_empty()) {
        r.push(ValidationIssue::error("status", "eligibility resource needs status", "elig-status"));
    }
    if v.pointer("/patient/reference").is_none() && v.pointer("/subscriber/reference").is_none() {
        r.push(ValidationIssue::error("patient", "patient or subscriber reference required", "elig-subject"));
    }
    Ok(r)
}

fn validate_271_sketch(text: &str) -> ValidationReport {
    let mut r = ValidationReport::ok();
    if !text.contains("ISA*") {
        r.push(ValidationIssue::error("x12", "271 sketch expects ISA envelope segment", "isa"));
    }
    if !text.contains("EB*") {
        r.push(ValidationIssue::error("x12", "271 response sketch expects EB benefit segment", "eb-segment"));
    }
    r
}
