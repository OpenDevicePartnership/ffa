# Copilot Instructions

> **Note:** The repository root [`AGENTS.md`](../AGENTS.md) is the canonical,
> superset guide for AI coding agents (GitHub Copilot, Claude, Cursor, Aider,
> etc.). The content below is preserved for tools that look here first; if
> the two files ever disagree, `AGENTS.md` wins. Please read `AGENTS.md` for
> build, lint, test, and contribution conventions.

## Commit Messages
- Subject line: capitalized, 50 characters or less, imperative mood (e.g., "Fix bug" not "Fixed bug")
- Separate subject from body with a blank line
- Wrap body text at 72 characters
- Use the body to explain *what* and *why*, not *how*

## AI Attribution
Every commit that includes AI-generated or AI-assisted work **must** contain an `Assisted-by` trailer in the commit message:
```
Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
```
Where:
- `AGENT_NAME` is the name of the AI tool or framework (e.g., `GitHub Copilot`)
- `MODEL_VERSION` is the specific model version used (e.g., `claude-opus-4.6`)
- `[TOOL1] [TOOL2]` are optional specialized analysis tools used (e.g., `coccinelle`, `sparse`, `smatch`, `clang-tidy`)
Basic development tools (git, cargo, editors) should not be listed.
AI agents **must** verify their own identity (agent name and model version) before composing the `Assisted-by` trailer — do not assume or hard-code a model name from a previous session.
AI agents **MUST NOT** add `Signed-off-by` tags. Only humans can certify the Developer Certificate of Origin.
