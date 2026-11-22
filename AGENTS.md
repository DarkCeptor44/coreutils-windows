# AGENTS.md

Instructions and guidelines for AI agents working in this repository.

## 1. Operating Rules for Agents

- **Advisory Only (Default)**: **Do NOT modify, edit, or create files unless explicitly commanded by the user.** The user is actively editing and maintaining the codebase.
- **No Git Operations**: **Agents must NEVER run `git commit`, `git add`, `git push`, `cog commit`, or alter Git repository state under any circumstances.** Staging, committing, tagging, branching, and pushing are strictly reserved for the user. An agent's Git role is exclusively advisory (e.g., suggesting commit messages or breaking down changes when asked).
- **Primary Role**: Act as an advisor and assistant when requested:
  - Suggest commit messages adhering to repository conventions.
  - Debug issues, review code, explain logic, and evaluate architecture or performance tradeoffs.
  - Answer specific questions directly and concisely without unsolicited modifications.
- **Never Modify Unrelated Code**: If explicitly instructed to edit a specific function or block, touch only what was requested. Never reformat, adjust whitespace, or alter unrelated code.
- **Re-read Before Modifying**: If explicitly instructed to edit files, always re-read them immediately prior to applying changes, as the user frequently edits code while chatting.

---

## 2. Commit Message Standards

This repository enforces Conventional Commits managed via **Cocogitto** (`cog.toml`).

- **Format**: `<type>(<scope>): <description>` or `<type>: <description>`
- **Allowed Types**:
  - `feat`: User-facing or library feature additions.
  - `fix`: Bug fixes.
  - `perf`: Performance optimizations.
  - `refactor`: Code changes that neither fix bugs nor add features.
  - `test`: Adding or updating tests / benchmarks.
  - `chore`: Maintenance, dependencies, metadata, or tooling.
  - `docs`: Documentation updates.
- **Scope Conventions**:
  - For changes isolated to a specific workspace member crate, use the **crate's package name** as the scope (e.g., `feat(<crate>): ...`, `fix(<crate>): ...`).
  - For root-level or cross-workspace changes, omit the scope (e.g., `chore: update workspace lints`).
  - Do not hardcode a static list of crate scopes in this file; inspect the workspace members when needed.
- **Style Rules**:
  - Lowercase description.
  - Imperative mood (e.g., `handle empty pathext entries` instead of `handled ...`).
  - No trailing period.
  - Keep commits small, atomic, and focused on a single logical change.

---

## 3. Repository & Workspace Philosophy

- **Structure**: A Cargo workspace containing multiple independent utility crates with independent versioning. Avoid listing individual crate names, file layouts, or per-crate CLI flags here so this document stays accurate as crates are added or changed.
- **Platform Priority**: Windows-first utilities that remain cross-platform where practical. Do not introduce Unix-only assumptions that break Windows builds or behavior.
- **Minimal Dependencies**: Prefer `std` over third-party crates whenever standard library functionality suffices. Keep dependency graphs lean.

---

## 4. Rust Style & Technical Constraints

- **Strictly Safe Rust**: `unsafe_code = "forbid"` is enforced at the workspace level. Never write or suggest direct `unsafe` code.
- **Lints & Documentation**: Respect `[workspace.lints]` (including `clippy::pedantic`). Enforce `missing_docs` and `missing_debug_implementations` on library crates, while allowing binary crates to override them.
- **Imports**: Import types, traits, and functions at the top of the file after global attributes (`use foo::Bar;`), rather than using inline path qualifications (`foo::Bar::new()`).
- **Module Hierarchy**: When creating new submodules, prefer the directory layout with a `mod.rs` file (`folder/mod.rs`).
- **Signatures**: Take references for parameters and return owned data where practical. Prioritize high-level, idiomatic abstractions over manual low-level plumbing.
