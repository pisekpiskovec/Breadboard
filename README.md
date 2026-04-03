# Breadboard

---

An ATmega16 emulator.

---

Currently work-in-progress.

Compiles on Linux. Windows and Haiku not tested. Depends on `crate` and `just`.

To build: `just build`
Or install to home's bin: `just install` (Linux-only)

Check [supported instructions](/INSTRUCTIONS.md) before running any program.

---

## Keyboard Shortcuts

| Shortcut | Action                     |
| -------- | -------------------------- |
| F5       | Auto Run toggle            |
| F8       | Step                       |
| F12      | Open Config                |
| F12      | Close Config               |
| Ctrl+F12 | Save and Close Config      |
| Ctrl+o   | Open Hex file              |
| Alt+o    | Open Binary file           |
| Ctrl+r   | Reset program              |
| Alt+r    | Restart emulator           |
| \*       | Toggle ASCII view in flash |
| Ctrl+q   | Quit                       |
