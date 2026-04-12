//! MCP tool definitions for the agents catalog extension.

use convergio_types::extension::McpToolDef;
use serde_json::json;

pub fn agents_tools() -> Vec<McpToolDef> {
    vec![
        McpToolDef {
            name: "cvg_list_agent_catalog".into(),
            description: "List agent definitions with optional filters.".into(),
            method: "GET".into(),
            path: "/api/agents/catalog".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "category": {"type": "string", "description": "Filter by category"},
                    "status": {"type": "string", "description": "Filter by status"},
                    "name": {"type": "string", "description": "Filter by name"},
                    "limit": {"type": "integer", "description": "Max results (default 20)"}
                }
            }),
            min_ring: "sandboxed".into(),
            path_params: vec![],
        },
        McpToolDef {
            name: "cvg_get_agent".into(),
            description: "Get details for a specific agent by name.".into(),
            method: "GET".into(),
            path: "/api/agents/catalog/:name".into(),
            input_schema: json!({
                "type": "object",
                "properties": {"name": {"type": "string"}},
                "required": ["name"]
            }),
            min_ring: "community".into(),
            path_params: vec!["name".into()],
        },
    ]
}
