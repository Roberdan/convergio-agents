//! Seed data: business_operations + leadership_strategy agents (18 agents).

use crate::types::{AgentCategory, AgentInput};

fn biz(name: &str, role: &str, tier: &str, esc: Option<&str>, caps: &[&str]) -> AgentInput {
    AgentInput {
        name: name.into(),
        role: role.into(),
        org: "convergio".into(),
        category: AgentCategory::BusinessOperations,
        model_tier: tier.into(),
        max_tokens: 200_000,
        hourly_budget: 0.0,
        capabilities: caps.iter().map(|s| s.to_string()).collect(),
        prompt_ref: None,
        escalation_target: esc.map(Into::into),
    }
}

fn lead(name: &str, role: &str, tier: &str, esc: Option<&str>, caps: &[&str]) -> AgentInput {
    AgentInput {
        name: name.into(),
        role: role.into(),
        org: "convergio".into(),
        category: AgentCategory::LeadershipStrategy,
        model_tier: tier.into(),
        max_tokens: 200_000,
        hourly_budget: 0.0,
        capabilities: caps.iter().map(|s| s.to_string()).collect(),
        prompt_ref: None,
        escalation_target: esc.map(Into::into),
    }
}

pub fn agents() -> Vec<AgentInput> {
    let mut v = vec![
        biz(
            "andrea",
            "Sales and partnerships manager",
            "t2",
            Some("ali-orchestrator"),
            &["sales", "partnerships", "business-development"],
        ),
        biz(
            "sofia",
            "Customer success manager",
            "t2",
            Some("ali-orchestrator"),
            &["customer-success", "onboarding", "retention"],
        ),
        biz(
            "marcello",
            "Marketing strategist",
            "t2",
            Some("ali-orchestrator"),
            &["marketing", "content-strategy", "growth"],
        ),
        biz(
            "davide",
            "Product analytics lead",
            "t2",
            Some("ali-orchestrator"),
            &["analytics", "product-metrics", "data-analysis"],
        ),
        biz(
            "enrico",
            "Business development",
            "t2",
            Some("ali-orchestrator"),
            &["business-development", "market-research", "strategy"],
        ),
        biz(
            "fabio",
            "Operations manager",
            "t2",
            Some("ali-orchestrator"),
            &["operations", "process-optimization", "logistics"],
        ),
        biz(
            "luke",
            "Growth hacker",
            "t2",
            Some("ali-orchestrator"),
            &["growth", "experimentation", "funnel-optimization"],
        ),
        biz(
            "anna",
            "HR and people ops",
            "t2",
            Some("ali-orchestrator"),
            &["hr", "people-ops", "hiring", "culture"],
        ),
        biz(
            "dave",
            "Technical writer",
            "t2",
            Some("ali-orchestrator"),
            &[
                "technical-writing",
                "documentation",
                "api-docs",
                "tutorials",
            ],
        ),
        biz(
            "steve",
            "Community manager",
            "t2",
            Some("ali-orchestrator"),
            &["community", "support", "engagement", "communications"],
        ),
        biz(
            "dev-rel",
            "Developer relations — GitHub, README, demos, onboarding, adoption",
            "t3",
            Some("ali-orchestrator"),
            &[
                "developer-relations",
                "documentation",
                "demos",
                "developer-experience",
            ],
        ),
    ];
    v.extend([
        lead(
            "ali-cos",
            "Chief of Staff — strategic oversight",
            "t4",
            None,
            &[
                "strategy",
                "oversight",
                "coordination",
                "executive-leadership",
            ],
        ),
        lead(
            "amy-cfo",
            "CFO — financial strategy and budgets",
            "t4",
            Some("ali-cos"),
            &[
                "finance",
                "budgeting",
                "financial-strategy",
                "cost-analysis",
            ],
        ),
        lead(
            "antonio",
            "CTO — technical vision and architecture",
            "t4",
            Some("ali-cos"),
            &["architecture", "technical-vision", "rust", "system-design"],
        ),
        lead(
            "dan",
            "VP Engineering — delivery and velocity",
            "t4",
            Some("antonio"),
            &[
                "engineering-management",
                "delivery",
                "velocity",
                "team-leadership",
            ],
        ),
        lead(
            "domik",
            "VP Product — roadmap and features",
            "t3",
            Some("ali-cos"),
            &["product-management", "roadmap", "feature-planning"],
        ),
        lead(
            "matteo",
            "VP Design — design vision and brand",
            "t3",
            Some("ali-cos"),
            &["design-leadership", "brand", "design-system", "ux-strategy"],
        ),
        lead(
            "satya",
            "CEO — company strategy and vision",
            "t4",
            None,
            &["strategy", "vision", "leadership", "decision-making"],
        ),
    ]);
    v
}
