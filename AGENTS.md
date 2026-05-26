# AGENTS.md

Guidance for AI coding agents (GitHub Copilot, Claude, Cursor, Aider, etc.) and
human contributors working in this repository.

This file is the **single source of truth** for agent behavior in `ffa`. It is
a superset of `.github/copilot-instructions.md`, which is preserved as a
pointer for backward compatibility with tools that look there first.

---

## 1. Repository overview

- **Name:** `ffa` (crate), repository `openDevicePartnership/ffa`.
- **Purpose:** A Rust crate implementing the **Arm Firmware Framework for
  Armv8-A (FF-A)** protocol as described in **DEN0077A — "Firmware Framework
  Arm A-profile"**. It is intended for **Rust secure-partition (SP) services
  running under Hafnium** (or another FF-A-compliant hypervisor / SPM).
- **Crate kind:** library, `#![no_std]` (except under `cfg(test)`).
- **License:** MIT (see `LICENSE`).
- **MSRV:** Rust **1.75** (enforced in CI; see `Cargo.toml` and
  `.github/workflows/check.yml`).
- **Rust edition:** 2021.
- **Pinned toolchain:** `rust-toolchain.toml` selects the `stable` channel by
  default and installs the target `aarch64-unknown-none-softfloat` plus
  components `rust-src`, `rustfmt`, `llvm-tools-preview`, `clippy`.
- **Primary target:** `aarch64-unknown-none-softfloat`.
- **Also documented for:** `aarch64-unknown-none` (see
  `[package.metadata.docs.rs]`).

This crate is **not an application**. It is meant to be pulled in as a
dependency of a secure-partition image. Do not add `main.rs`, runtime
bootstrap, panic handlers, or global allocators here.

---

## 2. Crate layout

```
src/
├── lib.rs        — FfaError, FfaFunctionId, FfaParams, Ffa facade, ffa_smc()
├── console/      — FFA_CONSOLE_LOG64 + println!/panic helpers
├── features/     — FFA_FEATURES
├── indirect/     — Indirect messaging via shared memory with the NWd
├── memory/       — FFA_MEM_RETRIEVE_REQ shared-memory setup
├── msg/          — FFA_MSG_SEND_DIRECT_REQ2 / RESP2
├── notify/       — FFA_NOTIFICATION_SET
├── rxtx/         — FFA_RXTX_MAP / FFA_RXTX_UNMAP
├── version/      — FFA_VERSION (currently 1.2)
└── yld/          — FFA_YIELD
```

Top-level configuration / policy files:

```
Cargo.toml             — crate metadata, deps, MSRV
rust-toolchain.toml    — pinned toolchain & components
rustfmt.toml           — formatting rules (nightly-only options used in CI)
deny.toml              — cargo-deny configuration
.github/workflows/     — check.yml, nostd.yml
.github/copilot-instructions.md  — short pointer; AGENTS.md is the superset
CONTRIBUTING.md, CODE_OF_CONDUCT.md, CODEOWNERS, SECURITY.md, LICENSE
```

There are currently **no `examples/`, no `tests/` directory, and no
`benches/`**. Unit tests, if any, live inline under `#[cfg(test)] mod tests`.

---

## 3. Build, lint, test — verified commands

The commands below have been **executed locally on this branch** and exit `0`
unless noted. CI runs the same commands; matching them locally is the fastest
way for an agent to validate a change before pushing.

### 3.1 Format

```sh
cargo fmt --check
```

`rustfmt.toml` sets `group_imports = "StdExternalCrate"` and
`imports_granularity = "Module"` which are **unstable** rustfmt options. On
stable they emit warnings ("can't set … unstable features are only available
in nightly channel") but the check still passes. **Do not remove these
options.** CI runs `cargo fmt --check` on stable as well and tolerates the
same warnings.

To actually apply those import rules locally, use nightly:

```sh
cargo +nightly fmt
```

### 3.2 Clippy (matches CI lint set exactly)

```sh
cargo clippy --target aarch64-unknown-none-softfloat --no-default-features -- \
  -F clippy::suspicious -F clippy::correctness -F clippy::perf -F clippy::style
```

CI runs on both `stable` and `beta` toolchains via `dtolnay/rust-toolchain`
plus `giraffate/clippy-action`. The lint flags above are the **exact**
`clippy_flags` from `.github/workflows/check.yml`. New warnings that fall in
those four categories will fail the build — treat them as errors locally too.

CI's clippy job does **not** pass an explicit `--target`; on Linux it lints
against the host. Locally on Windows the host has no `std`-less SMC asm, so
prefer `--target aarch64-unknown-none-softfloat --no-default-features` to
mirror how the code is actually compiled in production.

### 3.3 Host check (MSRV job)

```sh
cargo check
```

The `msrv` job pins to Rust 1.75 (`matrix.msrv: ["1.75"]`) and just runs
`cargo check`. If you bump MSRV, update **both** `Cargo.toml`'s
`rust-version` and `check.yml`'s `matrix.msrv`.

### 3.4 no-std check (matches `nostd.yml`)

```sh
rustup target add aarch64-unknown-none-softfloat
cargo check --target aarch64-unknown-none-softfloat --no-default-features
```

This is the canonical way to validate that nothing pulls in `std`. The
toolchain file already includes this target, so `rustup target add` is a
no-op for the pinned toolchain.

### 3.5 Docs (nightly in CI)

```sh
RUSTDOCFLAGS=--cfg docsrs cargo +nightly doc --no-deps --all-features
```

A stable equivalent that still catches broken intra-doc links and missing
items is:

```sh
cargo doc --no-deps --target aarch64-unknown-none-softfloat --no-default-features
```

Use the nightly form before claiming "docs build" if you have touched
`#[cfg(...)]` gates or rustdoc attributes — `--cfg docsrs` flips
`#[cfg(docsrs)]` paths.

### 3.6 Feature powerset (`cargo hack`)

```sh
cargo install cargo-hack    # one-time
cargo hack --feature-powerset check
```

Features must be **additive**. There are currently no feature flags declared
in `Cargo.toml`, so this reduces to a single `cargo check`, but if you add
features, run the powerset locally.

### 3.7 Dependency policy (`cargo deny`)

```sh
cargo install cargo-deny    # one-time
cargo deny --manifest-path ./Cargo.toml check --all-features
```

`deny.toml` is the upstream template. Adjust it (do not silence it
blanket-style) when adding a dependency that trips advisories / licenses /
bans / sources.

### 3.8 Tests

The crate currently exposes **no integration tests** and the build is
configured with `#![cfg_attr(not(test), no_std)]`, so `cargo test` will try
to compile the host-side test harness with `std`. There is no CI test job
today. If you add unit tests, place them inline under
`#[cfg(test)] mod tests {}` and run them with the host toolchain
(`cargo test`), **not** with `--target aarch64-unknown-none-softfloat`.

---

## 4. Coding conventions

### 4.1 General Rust

- **`#![no_std]`** is required for non-test builds. Never reach for `std`,
  `alloc`, threads, files, sockets, environment variables, or `println!`
  from `std`. Use the in-crate `println!` (defined in `console`) for serial
  output, which routes through `FFA_CONSOLE_LOG64`.
- **No panics on the happy path.** Where you must encode an unreachable
  branch (e.g., the trailing arm in `impl From<u64> for FfaFunctionId`),
  keep `panic!("Unknown FfaFunctionId value")` localized and document it.
  Prefer returning `FfaError::UnknownError` / `FfaError::NotSupported` for
  data coming from the hypervisor.
- **Error type:** the crate-wide `Result<T> = core::result::Result<T,
  FfaError>`. Map raw `i64` results returned by SMC into `FfaError` via
  `FfaError::from(i64)` and prefer `.into_result()` to convert
  `FfaError::Ok` into `Ok(())`.
- **FF-A function IDs** live as a single `enum FfaFunctionId` in `lib.rs`,
  with bidirectional `From<u64>` / `Into<u64>` implementations. If you add
  a new FF-A call, add the variant **and** both arms; keep the IDs sorted
  in the order they appear in DEN0077A when reasonable.
- **SMC ABI:** use `FfaParams` (x0..x17) and the existing
  `ffa_smc(params: FfaParams) -> FfaParams` helper in `lib.rs`. Do not
  inline `asm!("smc #0", …)` in module code — always go through
  `ffa_smc()` so the architecture cfg and clobber list stay in one place.
- **Architecture-gated inline assembly:** the actual `smc #0` is guarded
  with `#[cfg(target_arch = "aarch64")]`. Preserve that gate so the crate
  continues to `cargo check` on the host for the MSRV job.
- **Public API:** every new public item gets a `///` rustdoc comment, even
  short. Keep `#![doc(html_root_url = "https://docs.rs/ffa/latest")]` in
  sync with the published version when releasing.

### 4.2 Formatting

- `max_width = 100`.
- `imports_granularity = "Module"` and `group_imports = "StdExternalCrate"`
  (nightly-only) — run `cargo +nightly fmt` before committing import
  changes so the ordering matches what reviewers will see.

### 4.3 Naming

- Types prefixed with `Ffa` (e.g., `FfaConsole`, `FfaMsg`, `FfaError`)
  matching the spec's `FFA_*` symbol convention but in Rust `CamelCase`.
- Module names are short, lowercase, and mirror the spec section (`msg`,
  `rxtx`, `notify`, `yld`, `memory`, `indirect`, `features`, `version`,
  `console`).
- Function-ID variants use `FfaXxxYyy` not `FFA_XXX_YYY` (the latter only
  appears in docs/comments to reference the spec).

### 4.4 Unsafe code

- The only place `unsafe` is currently needed is inline `asm!("smc #0", …)`
  inside `ffa_smc_inner`. Justify any new `unsafe` block with a `// SAFETY:`
  comment describing **why** the invariants hold (alignment, lifetime,
  exclusive access, hypervisor contract).

---

## 5. CI summary (`.github/workflows/`)

### 5.1 `check.yml`

Triggers: `push` to `main`, all `pull_request`. Concurrency-cancels stale
runs per PR.

Jobs:

| Job        | Toolchain      | Command                                                                                  |
|------------|----------------|------------------------------------------------------------------------------------------|
| `fmt`      | stable         | `cargo fmt --check`                                                                      |
| `clippy`   | stable + beta  | `cargo clippy -- -F clippy::suspicious -F clippy::correctness -F clippy::perf -F clippy::style` (via `giraffate/clippy-action@v1`) |
| `doc`      | nightly        | `RUSTDOCFLAGS=--cfg docsrs cargo doc --no-deps --all-features`                           |
| `hack`     | stable         | `cargo hack --feature-powerset check`                                                    |
| `deny`     | stable         | `cargo deny --manifest-path ./Cargo.toml check --all-features` (via `EmbarkStudios/cargo-deny-action@v2`) |
| `msrv`     | 1.75           | `cargo check`                                                                            |

A commented-out `semver` job is present for use after the first crates.io
release — do **not** uncomment without coordinating a release.

### 5.2 `nostd.yml`

Triggers: `push` to `main`, all `pull_request`.

| Job     | Target                              | Command                                                                            |
|---------|-------------------------------------|------------------------------------------------------------------------------------|
| `nostd` | `aarch64-unknown-none-softfloat`    | `cargo check --target aarch64-unknown-none-softfloat --no-default-features`        |

If you add another supported `no_std` target, expand the matrix in
`nostd.yml` and the `[package.metadata.docs.rs] targets = [...]` list in
`Cargo.toml` together.

---

## 6. Workflow for AI agents

1. **Always start clean.** `git status` before editing. Work on a feature
   branch off `upstream/main` (or `origin/main`); never commit to `main`
   directly.
2. **Read first.** Skim `src/lib.rs` and the module you intend to change;
   FF-A IDs and `FfaError` mappings are easy to break silently.
3. **Match CI commands locally** before claiming success. The four
   high-signal commands are:

   ```sh
   cargo fmt --check
   cargo check --target aarch64-unknown-none-softfloat --no-default-features
   cargo clippy --target aarch64-unknown-none-softfloat --no-default-features -- \
     -F clippy::suspicious -F clippy::correctness -F clippy::perf -F clippy::style
   cargo doc --no-deps --target aarch64-unknown-none-softfloat --no-default-features
   ```

4. **Surgical diffs.** Touch only what the task requires. Do not reformat
   unrelated files; do not "drive-by" upgrade dependencies.
5. **Document new public surface.** Rustdoc the item, and if it maps to a
   spec section, cite the DEN0077A chapter/section.
6. **Verify before claiming.** If you assert "builds" / "tests pass" /
   "clippy clean", you must have actually run the relevant command in this
   working tree on this branch.
7. **Don't invent features.** This crate has no Cargo features today. If
   you add one, gate it cleanly, ensure `cargo hack --feature-powerset
   check` still passes, and update `docs.rs` metadata if relevant.
8. **Never touch `LICENSE`, `CODE_OF_CONDUCT.md`, `SECURITY.md`,
   `CODEOWNERS`** unless explicitly asked.

---

## 7. Commit conventions

(From `.github/copilot-instructions.md`, expanded.)

### 7.1 Subject

- Capitalized.
- ≤ 50 characters.
- Imperative mood ("Add", "Fix", "Refactor" — not "Added"/"Adds"/"Fixing").
- No trailing period.
- Prefer a scope prefix matching the module when natural, e.g.
  `msg: Handle direct response error path`.

### 7.2 Body

- Separate subject from body with a blank line.
- Wrap at 72 characters.
- Explain **what** and **why**, not **how** — the diff already shows how.
- Reference the FF-A spec section (e.g., "DEN0077A §11.2") when the change
  implements or fixes adherence to a specific behavior.

### 7.3 Trailers (in this order, blank line above the trailer block)

```
Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
```

- `AGENT_NAME` — e.g., `GitHub Copilot`.
- `MODEL_VERSION` — the specific model version. **AI agents MUST verify
  their own identity before composing this trailer.** Do not hard-code or
  reuse a value from a previous session.
- `[TOOL1] [TOOL2]` — optional specialized analyzers actually used
  (e.g., `coccinelle`, `clang-tidy`, `miri`). Do **not** list basic tools
  like `git`, `cargo`, or your editor.
- **AI agents MUST NOT add `Signed-off-by`.** Only humans can certify the
  DCO.

Example:

```
msg: Return InvalidParameters on zero-length payload

The direct-response path silently truncated empty payloads to a
single null byte, which made it indistinguishable from an explicit
one-byte zero from the caller. Return FfaError::InvalidParameters
instead so callers can react.

DEN0077A §11.4 specifies that the source partition is responsible
for non-empty payloads on FFA_MSG_SEND_DIRECT_RESP2; rejecting the
call here matches the spec and matches what Hafnium does today.

Assisted-by: GitHub Copilot:claude-opus-4.7
```

### 7.4 Authorship

Use a per-commit identity, never `--global`:

```sh
git -c user.name="Your Name" -c user.email="you@example.com" commit ...
```

---

## 8. Pull requests

- Open PRs against `openDevicePartnership/ffa`'s `main` branch from a topic
  branch on a fork.
- Keep PRs focused — one logical change per PR. Split refactors from
  feature work.
- Ensure all six CI jobs (`fmt`, `clippy` × stable+beta, `doc`, `hack`,
  `deny`, `msrv`) and `nostd` are green before requesting review. The
  fastest local proxy is the four-command block in §6.3.
- Do **not** force-push to a shared branch once review has begun unless a
  maintainer asks for it.
- PR description: what changed, why, which FF-A behavior is affected, and
  any spec citation.

---

## 9. Dependency hygiene

- Default-features off for embedded deps (see `uuid = { version = "1.0",
  default-features = false, features = ["v1"] }` as the canonical
  example).
- `cargo deny check --all-features` must remain clean.
- Adding a dependency that pulls in `std` transitively is a regression —
  re-run `cargo check --target aarch64-unknown-none-softfloat
  --no-default-features` to confirm.

---

## 10. Things this repo deliberately does *not* do

- No `examples/`, no `benches/`, no `tests/` directory.
- No async runtime, no allocator, no panic handler — those belong in the
  consuming secure-partition image, not in this library.
- No `unsafe` outside the SMC asm shim.
- No platform-specific code for non-aarch64 targets beyond the `cfg`-gated
  asm fallback that lets `cargo check` work on the host.

If a request asks you to add any of these, push back and confirm with the
requester before proceeding.

---

## 11. Pointer

`.github/copilot-instructions.md` exists for tools that look there by
default. Its content is a strict subset of this file. **If the two ever
disagree, this file (`AGENTS.md`) wins.** Update both when changing the
commit-message or AI-attribution rules.
