# Security Policy

**Catwalk** (`catwalk`)

## Reporting a vulnerability

Email security reports privately to **John Anthony Mariquit <john@mariquit.com>**.
Use the subject `SECURITY: catwalk - <short summary>`. Do not open a public issue,
pull request, or discussion for a suspected vulnerability. If you want to
encrypt your report, first ask for a key in a message containing no details.

### What to include

- The `catwalk --version` output, operating system, and terminal emulator.
- The affected operation, such as viewing a board, saving an annotation, running
  `catwalk setup`, or installing a release. For setup, include the agent target.
- Impact, prerequisites, and the smallest sanitized example that demonstrates
  the issue. Remove secrets, private notes, and credentials.

### What to expect

The maintainer aims to meet these targets on a best-effort basis:

| Stage | Target |
| --- | --- |
| Acknowledgement | Within 5 working days |
| Initial assessment | Within 14 days |
| Fix or documented mitigation | Depends on severity |

Allow time for a fix before disclosing publicly. If you have a disclosure
deadline, include it in your first report. The maintainer will credit you for
accepted reports unless you ask not to be named.

## Supported versions

Catwalk is pre-1.0. Security fixes target `main`; earlier release tags do not
receive backports.

## Scope

Catwalk is a local terminal viewer and annotation editor. `catwalk setup` installs
the binary and guidance skill and removes recognized legacy LOMO configuration.
The separate `install.sh` script downloads and verifies release binaries.

### In scope

- Unexpected terminal behavior or disclosure caused by displaying Markdown or
  annotations.
- Annotation saves that modify unrelated content or write to an unintended file.
- Setup or installer writes that overwrite unrelated binaries or configuration.
- Release verification failures that allow a binary to be installed despite a
  failed or missing checksum check.

### Out of scope

- Agent behavior, including how an agent interprets task text or human notes.
  Catwalk displays the file; it does not execute tasks or enforce permissions.
- Dependency vulnerabilities without a Catwalk-specific impact. Report them to the
  dependency's maintainers and let Catwalk know if an update is needed.
