//! Seed data: core_utility agents (20 agents).

use crate::types::{AgentCategory, AgentInput};

fn a(name: &str, role: &str, tier: &str, esc: Option<&str>, caps: &[&str]) -> AgentInput {
    AgentInput {
        name: name.into(),
        role: role.into(),
        org: "convergio".into(),
        category: AgentCategory::CoreUtility,
        model_tier: tier.into(),
        max_tokens: 200_000,
        hourly_budget: 0.0,
        capabilities: caps.iter().map(|s| s.to_string()).collect(),
        prompt_ref: None,
        escalation_target: esc.map(Into::into),
    }
}

pub fn agents() -> Vec<AgentInput> {
    vec![
        a(
            "ali-orchestrator",
            "Chief of Staff — orchestrates all agent work",
            "t4",
            None,
            &["orchestration", "planning", "delegation", "coordination"],
        ),
        a(
            "thor",
            "Validation guardian — reviews all submitted work",
            "t4",
            Some("ali-orchestrator"),
            &[
                "validation",
                "gate-enforcement",
                "quality-review",
                "code-review",
            ],
        ),
        a(
            "convergio-pm",
            "Convergio Project Manager — audits plans, tracks costs, extracts learnings",
            "t3",
            Some("ali-orchestrator"),
            &[
                "project-management",
                "cost-tracking",
                "reporting",
                "planning",
            ],
        ),
        a(
            "wanda",
            "Workspace manager — worktree lifecycle",
            "t2",
            Some("ali-orchestrator"),
            &["workspace", "git", "worktree", "cleanup"],
        ),
        a(
            "sentinel",
            "Security sentinel — monitors for threats",
            "t3",
            Some("ali-orchestrator"),
            &["security", "threat-detection", "audit", "access-control"],
        ),
        a(
            "taskmaster",
            "Task queue manager — assigns and tracks tasks",
            "t2",
            Some("ali-orchestrator"),
            &["task-management", "scheduling", "queue", "assignment"],
        ),
        a(
            "socrates",
            "Socratic reviewer — asks probing questions",
            "t3",
            Some("thor"),
            &["code-review", "architecture-review", "critical-analysis"],
        ),
        a(
            "diana",
            "Dependency analyst — tracks inter-module deps",
            "t2",
            Some("ali-orchestrator"),
            &[
                "dependency-analysis",
                "module-graph",
                "breaking-change-detection",
            ],
        ),
        a(
            "marcus",
            "Metrics collector — gathers system telemetry",
            "t1",
            Some("ali-orchestrator"),
            &["metrics", "telemetry", "monitoring", "observability"],
        ),
        a(
            "xavier",
            "Context optimizer — compacts prompts and context",
            "t3",
            Some("ali-orchestrator"),
            &[
                "context-optimization",
                "prompt-engineering",
                "token-management",
            ],
        ),
        a(
            "po",
            "Product owner — prioritizes backlog items",
            "t3",
            Some("ali-orchestrator"),
            &["product-management", "backlog", "prioritization", "roadmap"],
        ),
        a(
            "plan-reviewer",
            "Plan review — validates plan structure",
            "t3",
            Some("thor"),
            &["plan-review", "validation", "structure-analysis"],
        ),
        a(
            "plan-business-advisor",
            "Business advisor for plan feasibility",
            "t3",
            Some("ali-orchestrator"),
            &["business-analysis", "feasibility", "risk-assessment"],
        ),
        a(
            "plan-post-mortem",
            "Post-mortem analyst — learns from failures",
            "t3",
            Some("ali-orchestrator"),
            &["post-mortem", "root-cause-analysis", "incident-review"],
        ),
        a(
            "compliance-validator",
            "Validates compliance rules in code",
            "t2",
            Some("elena"),
            &["compliance", "rule-validation", "code-audit"],
        ),
        a(
            "context-optimizer",
            "Optimizes context window usage",
            "t2",
            Some("xavier"),
            &["context-optimization", "compression", "summarization"],
        ),
        a(
            "deep-repo-auditor",
            "Deep audit of repository structure",
            "t3",
            Some("ali-orchestrator"),
            &["repo-audit", "architecture-analysis", "code-quality"],
        ),
        a(
            "design-validator",
            "Validates UI against design system",
            "t2",
            Some("jony"),
            &["design-system", "ui-validation", "component-review"],
        ),
        a(
            "doc-validator",
            "Validates documentation completeness",
            "t2",
            Some("ali-orchestrator"),
            &["documentation", "completeness-check", "api-docs"],
        ),
        a(
            "strategic-planner",
            "Long-term strategic planning",
            "t4",
            Some("ali-orchestrator"),
            &["strategy", "long-term-planning", "architecture", "vision"],
        ),
    ]
}
