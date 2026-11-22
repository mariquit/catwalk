# Catwalk workflow

The agent owns `TODO.md`. Catwalk displays it and lets a human edit one field:
the trailing annotation on a task. An annotation is feedback, not a command,
approval gate, or automatic task update.

## Ownership

| Actor | Writes | Responsibility |
| --- | --- | --- |
| Agent | `TODO.md` using normal file tools | Create the board, add context, and maintain tasks, IDs, statuses, and priorities. |
| Human through Catwalk | `[annotation]…[/annotation]` on an existing task | Suggest changes, ask questions, or flag work for review. |
| Catwalk viewer | Nothing during viewing or reload | Display the current file and refresh after changes. |

## Agent and human handoff

1. Run `catwalk setup <agent>` using an agent name from [INSTALL.md](INSTALL.md).
   Ask the agent to load the `catwalk` skill explicitly before creating the board.
   Check that the agent creates `TODO.md` with unique six-hex task IDs using the
   [documented notation](README.md#task-notation).
2. Run `catwalk` from the board directory, or `catwalk --file path/TODO.md` for a
   different file. Confirm the expected tasks and context are displayed before
   leaving feedback.
3. Select a task and press `n` to edit its note. Enter the contents without the
   reserved markers. Use `Alt+Enter` for a new line and `Enter` to save. Check
   that the note appears on the selected task and its status and priority still
   match the board.
4. Ask the agent to reread `TODO.md` when feedback is ready. The agent reads the
   note and makes any agreed task changes directly. Confirm the resulting edit
   in Catwalk; press `r` if the view has not refreshed.

## Write boundaries

Catwalk reads the current file before saving, locates the task by ID, and changes
only its trailing annotation. It does not create a board, assign IDs, modify
task text, or change status or priority. The agent owns progress tracking,
test execution, and Git commits. Status markers are descriptive; Catwalk does not
enforce them.

## Recovery

- If the board is missing, confirm the path or ask the agent to create it.
  Reopen Catwalk once the expected file exists.
- If an annotation cannot be saved because its task ID is missing or duplicated,
  ask the agent to fix the ID, reload, and check the selected task before retrying.
  For malformed annotations, have the agent repair the delimiters in the file.
- Avoid simultaneous writes. The viewer rereads before saving but does not lock
  the board. Coordinate with the agent and check the file after saving if both
  were editing it at the same time.
