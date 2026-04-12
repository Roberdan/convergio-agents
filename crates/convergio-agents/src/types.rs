//! Core types for the agent catalog.

use serde::{Deserialize, Serialize};

/// Classification of agents into functional categories.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentCategory {
    CoreUtility,
    TechnicalDevelopment,
    BusinessOperations,
    LeadershipStrategy,
    ComplianceLegal,
    SpecializedExperts,
    DesignUx,
    ReleaseManagement,
    ResearchReport,
}

impl AgentCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CoreUtility => "core_utility",
            Self::TechnicalDevelopment => "technical_development",
            Self::BusinessOperations => "business_operations",
            Self::LeadershipStrategy => "leadership_strategy",
            Self::ComplianceLegal => "compliance_legal",
            Self::SpecializedExperts => "specialized_experts",
            Self::DesignUx => "design_ux",
            Self::ReleaseManagement => "release_management",
            Self::ResearchReport => "research_report",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "core_utility" => Some(Self::CoreUtility),
            "technical_development" => Some(Self::TechnicalDevelopment),
            "business_operations" => Some(Self::BusinessOperations),
            "leadership_strategy" => Some(Self::LeadershipStrategy),
            "compliance_legal" => Some(Self::ComplianceLegal),
            "specialized_experts" => Some(Self::SpecializedExperts),
            "design_ux" => Some(Self::DesignUx),
            "release_management" => Some(Self::ReleaseManagement),
            "research_report" => Some(Self::ResearchReport),
            _ => None,
        }
    }
}

impl std::fmt::Display for AgentCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Catalog entry status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    Active,
    Disabled,
    Deprecated,
}

impl AgentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Disabled => "disabled",
            Self::Deprecated => "deprecated",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "active" => Some(Self::Active),
            "disabled" => Some(Self::Disabled),
            "deprecated" => Some(Self::Deprecated),
            _ => None,
        }
    }
}

/// Full agent specification — what the daemon needs to spawn an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    pub id: String,
    pub name: String,
    pub role: String,
    pub org: String,
    pub category: AgentCategory,
    /// Model tier preference: t1 (cheapest) to t4 (most capable).
    pub model_tier: String,
    pub max_tokens: i64,
    pub hourly_budget: f64,
    pub capabilities: Vec<String>,
    /// UUID of the prompt template in convergio-prompts.
    pub prompt_ref: Option<String>,
    /// Agent to escalate to when stuck or over budget.
    pub escalation_target: Option<String>,
    pub status: AgentStatus,
    pub created_at: String,
    pub updated_at: String,
}

/// Input for creating/updating an agent spec.
#[derive(Debug, Clone, Deserialize)]
pub struct AgentInput {
    pub name: String,
    pub role: String,
    #[serde(default = "default_org")]
    pub org: String,
    pub category: AgentCategory,
    #[serde(default = "default_tier")]
    pub model_tier: String,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: i64,
    #[serde(default)]
    pub hourly_budget: f64,
    #[serde(default)]
    pub capabilities: Vec<String>,
    pub prompt_ref: Option<String>,
    pub escalation_target: Option<String>,
}

/// Maximum length for free-text fields to prevent abuse.
const MAX_NAME_LEN: usize = 128;
const MAX_ROLE_LEN: usize = 512;
const MAX_ORG_LEN: usize = 128;
const MAX_CAPABILITIES: usize = 32;
const MAX_CAPABILITY_LEN: usize = 64;
const MAX_TOKENS_UPPER: i64 = 2_000_000;
const MAX_HOURLY_BUDGET: f64 = 10_000.0;
const VALID_TIERS: &[&str] = &["t1", "t2", "t3", "t4"];

impl AgentInput {
    /// Validate all fields. Returns `Err(reason)` on invalid input.
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() || self.name.len() > MAX_NAME_LEN {
            return Err(format!(
                "name must be 1–{MAX_NAME_LEN} chars, got {}",
                self.name.len()
            ));
        }
        // Only allow alphanumeric, hyphens, underscores in name (prevents injection in logs/paths)
        if !self
            .name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err("name must contain only [a-zA-Z0-9_-]".into());
        }
        if self.role.is_empty() || self.role.len() > MAX_ROLE_LEN {
            return Err(format!(
                "role must be 1–{MAX_ROLE_LEN} chars, got {}",
                self.role.len()
            ));
        }
        if self.org.is_empty() || self.org.len() > MAX_ORG_LEN {
            return Err(format!(
                "org must be 1–{MAX_ORG_LEN} chars, got {}",
                self.org.len()
            ));
        }
        if !VALID_TIERS.contains(&self.model_tier.as_str()) {
            return Err(format!(
                "model_tier must be one of {VALID_TIERS:?}, got '{}'",
                self.model_tier
            ));
        }
        if self.max_tokens <= 0 || self.max_tokens > MAX_TOKENS_UPPER {
            return Err(format!(
                "max_tokens must be 1–{MAX_TOKENS_UPPER}, got {}",
                self.max_tokens
            ));
        }
        if self.hourly_budget < 0.0 || self.hourly_budget > MAX_HOURLY_BUDGET {
            return Err(format!(
                "hourly_budget must be 0.0–{MAX_HOURLY_BUDGET}, got {}",
                self.hourly_budget
            ));
        }
        if self.capabilities.len() > MAX_CAPABILITIES {
            return Err(format!(
                "capabilities max {MAX_CAPABILITIES}, got {}",
                self.capabilities.len()
            ));
        }
        for cap in &self.capabilities {
            if cap.len() > MAX_CAPABILITY_LEN {
                return Err(format!(
                    "capability too long (max {MAX_CAPABILITY_LEN}): '{}'",
                    &cap[..cap.len().min(20)]
                ));
            }
        }
        Ok(())
    }
}

fn default_org() -> String {
    "convergio".into()
}
fn default_tier() -> String {
    "t2".into()
}
fn default_max_tokens() -> i64 {
    200_000
}

/// Query filters for listing agents.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct AgentQuery {
    pub category: Option<String>,
    pub status: Option<String>,
    pub name: Option<String>,
    pub limit: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_input() -> AgentInput {
        AgentInput {
            name: "test-agent".into(),
            role: "Test role".into(),
            org: "convergio".into(),
            category: AgentCategory::CoreUtility,
            model_tier: "t2".into(),
            max_tokens: 200_000,
            hourly_budget: 0.0,
            capabilities: vec!["test".into()],
            prompt_ref: None,
            escalation_target: None,
        }
    }

    #[test]
    fn category_roundtrip() {
        for cat in [
            AgentCategory::CoreUtility,
            AgentCategory::TechnicalDevelopment,
            AgentCategory::DesignUx,
            AgentCategory::ResearchReport,
        ] {
            let s = cat.as_str();
            assert_eq!(AgentCategory::parse(s), Some(cat));
        }
    }

    #[test]
    fn status_roundtrip() {
        for st in [AgentStatus::Active, AgentStatus::Disabled] {
            assert_eq!(AgentStatus::parse(st.as_str()), Some(st));
        }
    }

    #[test]
    fn category_display() {
        assert_eq!(AgentCategory::CoreUtility.to_string(), "core_utility");
    }

    #[test]
    fn validate_valid_input() {
        assert!(valid_input().validate().is_ok());
    }

    #[test]
    fn validate_empty_name() {
        let mut inp = valid_input();
        inp.name = String::new();
        assert!(inp.validate().is_err());
    }

    #[test]
    fn validate_name_special_chars() {
        let mut inp = valid_input();
        inp.name = "agent; DROP TABLE".into();
        assert!(inp.validate().is_err());
    }

    #[test]
    fn validate_name_too_long() {
        let mut inp = valid_input();
        inp.name = "a".repeat(MAX_NAME_LEN + 1);
        assert!(inp.validate().is_err());
    }

    #[test]
    fn validate_invalid_tier() {
        let mut inp = valid_input();
        inp.model_tier = "t99".into();
        assert!(inp.validate().is_err());
    }

    #[test]
    fn validate_negative_tokens() {
        let mut inp = valid_input();
        inp.max_tokens = -1;
        assert!(inp.validate().is_err());
    }

    #[test]
    fn validate_negative_budget() {
        let mut inp = valid_input();
        inp.hourly_budget = -5.0;
        assert!(inp.validate().is_err());
    }

    #[test]
    fn validate_too_many_capabilities() {
        let mut inp = valid_input();
        inp.capabilities = (0..MAX_CAPABILITIES + 1)
            .map(|i| format!("cap-{i}"))
            .collect();
        assert!(inp.validate().is_err());
    }
}
