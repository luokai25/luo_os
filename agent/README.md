# luo_os AI Agent Layer

The Python daemon that gives AI agents access to the luo_os kernel.

## How it works
```
[AI / Human]
     |
     | natural language
     v
[daemon.py] ←→ [luo_os kernel via serial COM1]
     |
     | JSON
     v
[commands.json / results.json]
```

## Start the daemon
```bash
# with real serial (QEMU running)
pip3 install pyserial
python3 agent/daemon.py

# mock mode (no serial needed for testing)
python3 agent/daemon.py --mock
```

## Send commands
```bash
python3 agent/send_command.py "list files"
python3 agent/send_command.py "read readme.txt"
python3 agent/send_command.py "show memory"
python3 agent/send_command.py "what processes are running"
```

## Natural language commands supported

| You say                  | OS command        |
|--------------------------|-------------------|
| list files               | ls                |
| read readme.txt          | cat readme.txt    |
| show memory              | meminfo           |
| what processes running   | ps                |
| how long has it been up  | uptime            |
| create notes.txt         | touch notes.txt   |
| delete test.txt          | rm test.txt       |
| write log.txt hello      | write log.txt hello |

## Connect to QEMU serial

When QEMU runs with `-serial stdio`, connect daemon to `/dev/pts/X`.
Find the PTY with: `ls /dev/pts/`
Then set SERIAL_PORT in daemon.py accordingly.
