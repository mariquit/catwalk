# Install Catwalk

Copyright (c) 2026 John Anthony Mariquit <john@mariquit.com>

Catwalk views and annotates an existing, agent-owned `TODO.md`. It does not create
the file or manage tasks, priorities, tests, or Git.

## Install the binary

```bash
curl -fsSL https://raw.githubusercontent.com/mariquit/catwalk/main/install.sh | sh
```

The installer downloads a tagged release and verifies its `.sha256` asset.
Release installation requires a published `v*` release with Catwalk assets.
Alternatively, download the matching binary from GitHub Releases, or build from source:

```bash
cargo install --path .
```

## Install guidance for your agent

```bash
catwalk setup         # OpenCode, the default
catwalk setup claude
catwalk setup codex
catwalk setup cursor
```

Setup writes only the guidance skill and installs/copies the binary. It does
not add plugins, hooks, persistent instruction blocks, or Cursor rules.

| Agent | Guidance skill |
| --- | --- |
| OpenCode | `~/.config/opencode/skills/catwalk/SKILL.md` (respects `XDG_CONFIG_HOME`) |
| Claude Code | `~/.claude/skills/catwalk/SKILL.md` |
| Codex | `~/.agents/skills/catwalk/SKILL.md` |
| Cursor | `~/.cursor/skills/catwalk/SKILL.md` |

The skill describes status, priority, IDs, rich context, and human comments.
Agents read and edit `TODO.md` directly. Skill loading depends on the agent;
invoke the `catwalk` skill explicitly if it is not picked up automatically.

Setup puts the binary in `~/.bun/bin` when that directory is on `PATH`, otherwise
`~/.local/bin`. On Windows it installs `catwalk.exe`. Ensure the destination is on
`PATH`; setup prints a reminder if it is not.

## Migrating from LOMO

1. Install Catwalk and run `catwalk --version` to verify the new command is on `PATH`.
2. Update any `LOMO_*` overrides to `CATWALK_*` before running `catwalk setup`
   for **each agent you previously configured**. Check the binary and skill paths
   printed by setup to confirm it used the intended locations.
3. Replace `lomo` command and skill references in your scripts and prompts with
   `catwalk`. Restart the agent and ask it to load the `catwalk` skill before asking
   for a todo list. Setup leaves the old `lomo` binary and skill in place.
4. Close any running LOMO viewer and open the same `TODO.md` in Catwalk to verify
   that the tasks and annotations are still visible. Use notes rather than task
   creation or priority-changing keys to suggest changes.

Setup removes recognized LOMO-owned hooks, the old OpenCode LOMO plugin, or an
owned Cursor rule. It does not read or write `AGENTS.md` or `CLAUDE.md`. Other
hooks, user instructions, and configuration are preserved. Invalid JSON or an
incomplete marked block is reported rather than overwritten.

Old `.git/lomo/` bookkeeping is no longer read or written. Setup does not delete
repository data or rewrite existing `TODO.md` files, including old annotations.

## Overrides

| Variable | Effect |
| --- | --- |
| `CATWALK_FILE` | Existing file to display instead of `TODO.md` |
| `CATWALK_BIN_DIR` | Binary installation directory |
| `CATWALK_OPENCODE_CONFIG` | OpenCode configuration directory |
| `CATWALK_CLAUDE_CONFIG` | Claude Code configuration directory |
| `CATWALK_CODEX_HOME` / `CODEX_HOME` | Codex configuration directory used for legacy cleanup |
| `CATWALK_CURSOR_CONFIG` | Cursor configuration directory |
| `CATWALK_AGENTS_SKILLS` | Codex skill directory instead of `~/.agents/skills` |
| `CATWALK_REPO` | GitHub repository used by `install.sh` instead of `mariquit/catwalk` |
| `XDG_CONFIG_HOME` | Base for OpenCode configuration |
