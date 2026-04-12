//! Seed data: technical_development agents (10 agents).

use crate::types::{AgentCategory, AgentInput};

fn a(name: &str, role: &str, tier: &str, esc: Option<&str>, caps: &[&str]) -> AgentInput {
    AgentInput {
        name: name.into(),
        role: role.into(),
        org: "convergio".into(),
        category: AgentCategory::TechnicalDevelopment,
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
            "baccio",
            "Executor — implements code changes from plans",
            "t3",
            Some("thor"),
            &["rust", "backend", "implementation", "code-review"],
        ),
        a(
            "rex",
            "Refactoring expert — improves code quality",
            "t3",
            Some("thor"),
            &["rust", "refactoring", "code-quality", "architecture"],
        ),
        a(
            "task-executor",
            "General task executor for plan steps",
            "t3",
            Some("thor"),
            &["rust", "implementation", "testing", "pr-creation"],
        ),
        a(
            "task-executor-tdd",
            "TDD executor — red-green-refactor",
            "t3",
            Some("thor"),
            &["rust", "tdd", "testing", "implementation"],
        ),
        a(
            "dario-debugger",
            "Debugger — investigates and fixes bugs",
            "t3",
            Some("baccio"),
            &["debugging", "rust", "root-cause-analysis", "profiling"],
        ),
        a(
            "marco-devops",
            "DevOps engineer — CI/CD and infra",
            "t2",
            Some("ali-orchestrator"),
            &[
                "ci-cd",
                "github-actions",
                "docker",
                "deployment",
                "infrastructure",
            ],
        ),
        a(
            "omri-data-scientist",
            "Data scientist — analytics and ML",
            "t3",
            Some("ali-orchestrator"),
            &["data-analysis", "machine-learning", "python", "statistics"],
        ),
        a(
            "otto-performance",
            "Performance engineer — profiling and opt",
            "t3",
            Some("baccio"),
            &[
                "performance",
                "profiling",
                "optimization",
                "benchmarking",
                "rust",
            ],
        ),
        a(
            "paolo-best-practices",
            "Best practices enforcer",
            "t2",
            Some("thor"),
            &["best-practices", "linting", "code-standards", "rust"],
        ),
        a(
            "adversarial-debugger",
            "Adversarial testing — finds edge cases",
            "t3",
            Some("thor"),
            &[
                "adversarial-testing",
                "fuzzing",
                "edge-cases",
                "security-testing",
            ],
        ),
    ]
}
