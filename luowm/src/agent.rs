/// AI Agent window control API
/// Agents can open/close/move/resize windows
/// and inject content via this interface

use crate::types::Rect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentCommand {
    OpenWindow   { title: String, x: i32, y: i32, w: u32, h: u32 },
    CloseWindow  { id: u32 },
    MoveWindow   { id: u32, x: i32, y: i32 },
    ResizeWindow { id: u32, w: u32, h: u32 },
    WriteContent { id: u32, lines: Vec<String> },
    AppendLine   { id: u32, line: String },
    ClearContent { id: u32 },
    FocusWindow  { id: u32 },
    ListWindows,
    RunCommand   { cmd: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentResponse {
    Ok,
    Error(String),
    WindowId(u32),
    WindowList(Vec<WindowInfo>),
    CommandOutput(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub id:    u32,
    pub title: String,
    pub rect:  [i32; 4],   // x,y,w,h
}

/// Agent command queue — thread safe
pub struct AgentQueue {
    pub commands: std::sync::Mutex<Vec<AgentCommand>>,
}

impl AgentQueue {
    pub fn new() -> Self {
        Self { commands: std::sync::Mutex::new(Vec::new()) }
    }

    pub fn push(&self, cmd: AgentCommand) {
        self.commands.lock().unwrap().push(cmd);
    }

    pub fn drain(&self) -> Vec<AgentCommand> {
        let mut q = self.commands.lock().unwrap();
        std::mem::take(&mut *q)
    }
}

/// Parse a JSON command string into an AgentCommand
pub fn parse_command(json: &str) -> Option<AgentCommand> {
    serde_json::from_str(json).ok()
}

/// Serialize a response to JSON
pub fn serialize_response(resp: &AgentResponse) -> String {
    serde_json::to_string(resp).unwrap_or_else(|_| "{}".to_string())
}
