# Project: Catwalk

**Catwalk** watches the agent’s `TODO.md` and leave feedback without taking over its tasks. The agent writes the context and maintains task IDs, statuses, and priorities. Catwalk defaults to OpenCode.

```text
  ┌───┬───┬───┬───┬───┬───┬───┐
  │ C │ A │ T │ W │ A │ L │ K │ v0.1.0
  └───┴───┴───┴───┴───┴───┴───┘
  Terminal viewer for agent-owned TODO lists
```

## What it does

- Displays the task tree, statuses, priorities, and task context.
- Refreshes when the agent changes the file.
- Supports folding, search, status/priority filters, and an inspector.
- Edits human annotations with `n`, preserving the existing task text.
- Provides a guidance skill describing the Markdown conventions.

Catwalk does **not** create `TODO.md`, generate IDs, add tasks, change statuses or priorities, enforce a lifecycle, run tests, make commits, or orchestrate agents.

It has no task-management CLI, hooks, session state, or Git integration.


## Related OpenCode projects

See [Structural](https://john.mariquit.com/#tools) for a map of these tools and how they fit into an OpenCode workflow.

- [Trestle](https://github.com/mariquit/trestle): Software engineering skills for OpenCode
- [Truss](https://github.com/mariquit/truss): Plugins for OpenCode
- [Lattice](https://github.com/mariquit/lattice): Semantic codebase wiring for OpenCode
- [Span](https://github.com/mariquit/span): Contextual aggregate memory mesh for OpenCode
- [Formwork](https://github.com/mariquit/formwork): Spec-driven agent for solo developers on OpenCode

## Human feedback

```markdown
# Login redesign

Context and implementation plans written by the agent belong here.

- [.] (p2) #a1b2c3 Replace the login flow [annotation]**Please prioritize the migration path first.** Consider p1; see [the design](design.md).[/annotation]
```

Humans suggest changes through notes. The agent reads/re-reads the file and
updates priority, progress, or status itself. Task creation and editing are
always the agent's responsibility, using its normal file tools rather than Catwalk.
The optional annotation is at the end of the task description, enclosed by
`[annotation]` and `[/annotation]`. Its contents can use Markdown. In the TUI,
the markers are hidden and the contents are displayed in yellow.

Press `n` to edit the annotation. The input box shows only its contents, not
the markers. `Enter` saves the contents with the markers restored, replacing
the existing annotation rather than adding a second one. Use `Alt+Enter` for a
new line or paste multiline Markdown; the closing marker ends that annotation.

## Task notation

```text
- [STATUS] [(pPRIORITY)] #hhhhhh Description [annotation]Markdown text[/annotation]
```

The agent chooses unique six-hex IDs and keeps each ID with its task. Priority
is optional (`p0`–`p4`); omitted priority means `p2`. The entire annotation field
is optional. The marker strings are reserved delimiters, not part of the note's
editable contents. Nest tasks with indentation.
Headings, prose, code examples, and other Markdown provide rich context.

| Status | Meaning |
| --- | --- |
| `[ ]` | Pending |
| `[.]` | In progress |
| `[:]` | In review |
| `[+]` | Discovered work |
| `[?]` | Blocked |
| `[!]` | Sensitive work needing human confirmation |
| `[x]` | Completed |
| `[~]` | Canceled |

These statuses are displayed, not enforced. Lines without IDs are shown as-is;
the agent needs to assign IDs before humans can annotate those tasks.

## Use

```bash
catwalk                      # View the agent's existing TODO.md
catwalk --file path/TODO.md   # View another existing file
catwalk setup                # Install the guidance skill for OpenCode
catwalk setup claude         # Or: codex, cursor
```

`CATWALK_FILE` also selects the file. Catwalk never creates a missing board.

| Key | Action |
| --- | --- |
| `j` / `k`, arrows | Move |
| `h` / `l`, arrows | Collapse / expand |
| `n` | Edit annotation contents, including priority suggestions or cancellation requests |
| `Alt+Enter` in a note | New line (`Enter` saves the note) |
| `/` | Search/filter (`p:0`, `status:blocked`, or text) |
| `Tab`, `J` / `K` | Scroll the inspector |
| `r` | Reload |
| `?` | Help |
| `q` / `Esc` | Quit |

There are no task creation, status, or priority-changing keys.

## Guidance and installation

[`assets/SKILL.md`](assets/SKILL.md) is the embedded guidance skill. It tells the
agent to own the file, maintain the notation, read human comments, and consider
an atomic Git commit when completing a task. It does not impose a CLI workflow.

See [GETTING_STARTED.md](GETTING_STARTED.md) before the first todo list. The `catwalk` skill is not loaded automatically.

See [INSTALL.md](INSTALL.md) for installation and upgrading older setups.
See [WORKFLOW.md](WORKFLOW.md) for the agent and human handoff.

## Related OpenCode projects

- [Trestle](https://github.com/mariquit/trestle): Software engineering skills for OpenCode
- [Truss](https://github.com/mariquit/truss): Plugins for OpenCode
- [Formwork](https://github.com/mariquit/formwork): Spec-driven agent for solo developers on OpenCode
- [Lattice](https://github.com/mariquit/lattice): Semantic codebase wiring for OpenCode
- [Span](https://github.com/mariquit/span): Contextual aggregate memory mesh for OpenCode

## License

Copyright (c) 2026 John Anthony Mariquit <john@mariquit.com>

Catwalk is licensed under the [Apache License, Version 2.0](LICENSE).
See [NOTICE](NOTICE) for attribution and [TRADEMARK.md](TRADEMARK.md) for naming guidance.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for build checks and [DCO](DCO) sign-off.
Follow the [Code of Conduct](CODE_OF_CONDUCT.md) and report vulnerabilities
privately as described in [SECURITY.md](SECURITY.md).
