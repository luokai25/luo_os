#!/usr/bin/env python3
"""
Send a command to the luo_os AI agent daemon
Usage: python3 agent/send_command.py "list files"
       python3 agent/send_command.py "read readme.txt"
       python3 agent/send_command.py "memory info"
"""
import sys
import json
import uuid
import time
import os

CMD_FILE    = "agent/commands.json"
RESULT_FILE = "agent/results.json"

def send(cmd):
    cmd_id = str(uuid.uuid4())[:8]
    with open(CMD_FILE, "w") as f:
        json.dump({"cmd": cmd, "id": cmd_id}, f)
    print(f"[send] Sent: {cmd} (id={cmd_id})")

    for _ in range(20):
        time.sleep(0.25)
        if os.path.exists(RESULT_FILE):
            with open(RESULT_FILE) as f:
                result = json.load(f)
            if result.get("id") == cmd_id:
                print(f"[result]\n{result.get('result','')}")
                return
    print("[timeout] No response from daemon")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 agent/send_command.py <command>")
        sys.exit(1)
    send(" ".join(sys.argv[1:]))
