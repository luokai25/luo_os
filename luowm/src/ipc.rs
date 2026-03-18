/// IPC — communicates with luo_os kernel over serial
/// Sends shell commands, receives output

use std::sync::{Arc, Mutex};

pub struct KernelIpc {
    pub connected: bool,
    pub port:      String,
    pub output_buf: Arc<Mutex<Vec<String>>>,
}

impl KernelIpc {
    pub fn new(port: &str) -> Self {
        Self {
            connected:  false,
            port:       port.to_string(),
            output_buf: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn connect(&mut self) -> bool {
        // In real deployment: open serialport here
        // For now: mock connection
        self.connected = true;
        println!("[IPC] Connected to kernel on {}", self.port);
        true
    }

    pub fn send_command(&self, cmd: &str) -> String {
        if !self.connected {
            return "[IPC] Not connected".to_string();
        }
        println!("[IPC] >> {}", cmd);
        // mock responses for testing
        match cmd {
            "ls"      => "  readme.txt  (91 bytes)\n  motd.txt  (35 bytes)".to_string(),
            "meminfo" => "  Total: 1048576 bytes\n  Used: 4096 bytes\n  Free: 1044480 bytes".to_string(),
            "ps"      => "  PID  NAME     STATE    TICKS\n  0    kernel   running  0".to_string(),
            "uptime"  => "  Uptime: 42 seconds".to_string(),
            "version" => "  luo_os v1.0".to_string(),
            _         => format!("  [kernel] ok: {}", cmd),
        }
    }

    pub fn get_output(&self) -> Vec<String> {
        self.output_buf.lock().unwrap().clone()
    }
}
