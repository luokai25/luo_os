#!/usr/bin/env python3
"""
luo_os AI Agent Daemon
Connects to the OS via serial port (COM1)
Provides natural language interface to the kernel
"""

import serial
import threading
import time
import sys
import os
import json
from datetime import datetime

# ── config ──────────────────────────────────────────────
SERIAL_PORT = "/dev/ttyS0"   # change to /dev/pts/X if using pty
BAUD_RATE   = 38400
LOG_FILE    = "agent/agent.log"
CMD_FILE    = "agent/commands.json"

# ── logger ───────────────────────────────────────────────
def log(msg):
    ts = datetime.now().strftime("%H:%M:%S")
    line = f"[{ts}] {msg}"
    print(line)
    with open(LOG_FILE, "a") as f:
        f.write(line + "\n")

# ── serial connection ─────────────────────────────────────
class SerialConnection:
    def __init__(self, port, baud):
        self.port   = port
        self.baud   = baud
        self.conn   = None
        self.buffer = ""

    def connect(self):
        try:
            self.conn = serial.Serial(
                self.port, self.baud,
                timeout=1,
                bytesize=serial.EIGHTBITS,
                parity=serial.PARITY_NONE,
                stopbits=serial.STOPBITS_ONE
            )
            log(f"Connected to {self.port} at {self.baud} baud")
            return True
        except Exception as e:
            log(f"Connection failed: {e}")
            return False

    def send(self, cmd):
        if not self.conn:
            return
        data = (cmd + "\r\n").encode()
        self.conn.write(data)
        log(f">> {cmd}")

    def readline(self):
        if not self.conn:
            return ""
        try:
            line = self.conn.readline().decode("utf-8", errors="replace").strip()
            if line:
                log(f"<< {line}")
            return line
        except:
            return ""

    def read_until_prompt(self, timeout=3.0):
        out = []
        start = time.time()
        while time.time() - start < timeout:
            line = self.readline()
            if "luo_os:~$" in line:
                break
            if line:
                out.append(line)
        return "\n".join(out)

    def close(self):
        if self.conn:
            self.conn.close()

# ── command executor ──────────────────────────────────────
class AgentExecutor:
    def __init__(self, serial_conn):
        self.serial = serial_conn

    def run(self, cmd):
        self.serial.send(cmd)
        return self.serial.read_until_prompt()

    def ls(self):
        return self.run("ls")

    def cat(self, filename):
        return self.run(f"cat {filename}")

    def write(self, filename, text):
        return self.run(f"write {filename} {text}")

    def touch(self, filename):
        return self.run(f"touch {filename}")

    def rm(self, filename):
        return self.run(f"rm {filename}")

    def append(self, filename, text):
        return self.run(f"append {filename} {text}")

    def ps(self):
        return self.run("ps")

    def meminfo(self):
        return self.run("meminfo")

    def uptime(self):
        return self.run("uptime")

    def version(self):
        return self.run("version")

# ── natural language parser ───────────────────────────────
class NLParser:
    def __init__(self, executor):
        self.ex = executor

    def parse(self, text):
        t = text.lower().strip()

        if "list" in t and "file" in t:
            return self.ex.ls()
        if t.startswith("read ") or t.startswith("cat ") or t.startswith("show "):
            fname = t.split()[-1]
            return self.ex.cat(fname)
        if t.startswith("create ") or t.startswith("touch "):
            fname = t.split()[-1]
            return self.ex.touch(fname)
        if t.startswith("delete ") or t.startswith("remove "):
            fname = t.split()[-1]
            return self.ex.rm(fname)
        if "memory" in t or "mem" in t:
            return self.ex.meminfo()
        if "process" in t or "ps" in t or "task" in t:
            return self.ex.ps()
        if "uptime" in t or "how long" in t:
            return self.ex.uptime()
        if "version" in t:
            return self.ex.version()
        if t.startswith("write "):
            parts = t.split(None, 2)
            if len(parts) >= 3:
                return self.ex.write(parts[1], parts[2])
        if t.startswith("append "):
            parts = t.split(None, 2)
            if len(parts) >= 3:
                return self.ex.append(parts[1], parts[2])

        return f"[AI] Unknown command: {text}\n[AI] Try: list files, read <file>, memory, ps, uptime"

# ── command file watcher ──────────────────────────────────
class CommandWatcher:
    """
    Watches agent/commands.json for new commands from external tools.
    Format: {"cmd": "list files", "id": "abc123"}
    Results written to agent/results.json
    """
    def __init__(self, parser):
        self.parser   = parser
        self.last_cmd = None
        self.running  = True

    def watch(self):
        while self.running:
            try:
                if os.path.exists(CMD_FILE):
                    with open(CMD_FILE) as f:
                        data = json.load(f)
                    cmd_id = data.get("id", "")
                    cmd    = data.get("cmd", "")
                    if cmd and cmd_id != self.last_cmd:
                        self.last_cmd = cmd_id
                        log(f"[WATCHER] New command: {cmd}")
                        result = self.parser.parse(cmd)
                        with open("agent/results.json", "w") as f:
                            json.dump({
                                "id":     cmd_id,
                                "cmd":    cmd,
                                "result": result,
                                "time":   datetime.now().isoformat()
                            }, f, indent=2)
                        log(f"[WATCHER] Result written")
            except Exception as e:
                log(f"[WATCHER] Error: {e}")
            time.sleep(0.5)

# ── interactive REPL ──────────────────────────────────────
def repl(parser):
    print("\n=== luo_os AI Agent REPL ===")
    print("Type natural language commands or 'quit' to exit\n")
    while True:
        try:
            text = input("agent> ").strip()
            if not text:
                continue
            if text in ("quit", "exit", "q"):
                break
            result = parser.parse(text)
            print(result)
        except (KeyboardInterrupt, EOFError):
            break

# ── main ──────────────────────────────────────────────────
def main():
    os.makedirs("agent", exist_ok=True)
    log("luo_os AI Agent Daemon starting...")

    serial_conn = SerialConnection(SERIAL_PORT, BAUD_RATE)

    if "--mock" in sys.argv:
        log("Running in MOCK mode (no serial connection)")
        class MockSerial:
            def send(self, cmd):    log(f"[MOCK] >> {cmd}")
            def readline(self):     return "luo_os:~$ "
            def read_until_prompt(self, timeout=3): return f"[MOCK response to command]"
        executor = AgentExecutor(MockSerial())
    else:
        if not serial_conn.connect():
            log("Failed to connect. Use --mock for testing without serial.")
            sys.exit(1)
        log("Waiting for luo_os prompt...")
        serial_conn.read_until_prompt(timeout=5)
        executor = AgentExecutor(serial_conn)

    parser  = NLParser(executor)
    watcher = CommandWatcher(parser)

    watcher_thread = threading.Thread(target=watcher.watch, daemon=True)
    watcher_thread.start()
    log("Command watcher started (watching agent/commands.json)")

    repl(parser)

    watcher.running = False
    serial_conn.close()
    log("Agent daemon stopped.")

if __name__ == "__main__":
    main()
