# Getting started

Copyright (c) 2026 John Anthony Mariquit <john@mariquit.com>

Catwalk views an agent-owned `TODO.md`. It does not create that file. The `catwalk` skill is not loaded automatically, so ask your agent to load it before asking for a todo list.

## 1. Install

Install the binary and the guidance skill. See [INSTALL.md](INSTALL.md).

```bash
catwalk setup
```

Use `catwalk setup claude`, `catwalk setup codex`, or `catwalk setup cursor` for another agent.

## 2. Load the skill

Ask your agent to load the skill before it creates a list. For example:

```agent
load the catwalk skill
```

```agent
load the catwalk skill before you create a todo list
```

## 3. Create the list

Once the skill is loaded, ask your agent to create a todo list. The agent writes `TODO.md` with its normal file tools. Catwalk does not create the file, assign IDs, or manage tasks.

## 4. View and annotate

```bash
catwalk
```

Press `n` to leave a note on the selected task. The agent reads the file and updates priority, progress, or status itself.
