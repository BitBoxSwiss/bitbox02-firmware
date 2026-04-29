# AGENTS.md

This file is the entry point for Codex and other coding agents working in this
repository. Keep it short, task-oriented, and linked to the detailed guidance in
`doc/agent/`. OpenAI recommends using AGENTS.md for durable repository context
such as layout, commands, engineering conventions, constraints, and verification
steps.
See OpenAI's AGENTS.md guidance and Codex best practices:

- https://developers.openai.com/codex/guides/agents-md
- https://developers.openai.com/codex/learn/best-practices

## Start Here

- Read the files below that match the task before making changes:
  - [doc/agent/repository.md](doc/agent/repository.md): repository layout,
    firmware architecture, and Rust crate boundaries.
  - [doc/agent/commands.md](doc/agent/commands.md): host commands, container
    commands, build targets, formatting, and test commands.
  - [doc/agent/coding.md](doc/agent/coding.md): C, Rust, Python, protobuf,
    FFI, and migration conventions.
  - [doc/agent/testing.md](doc/agent/testing.md): unit-test layout, Rust test
    style, and expected verification.
  - [doc/agent/review.md](doc/agent/review.md): review focus, commit and PR
    guidance, and change-discipline rules.
- Prefer `rg`/`rg --files` for repository search.
- Keep changes scoped to the task. Do not do unrelated refactors or formatting.
- Never commit changes unless the user explicitly asks for a commit.

## Critical Rules

- Run regular Unix commands such as `git`, `rg`, `grep`, `ls`, `find`, `sed`,
  and `cat` directly on the host.
- Use `./scripts/dev_exec.sh <command>` only for project-specific commands that
  depend on the project toolchain or compiler environment.
- Do not wrap `./scripts/dev_exec.sh` itself in `bash -lc`. If shell features are
  genuinely needed, pass an explicit shell as the command, for example
  `./scripts/dev_exec.sh bash -lc 'cat versions.json | jq'`.
- Never run multiple `make` commands in parallel. Run them sequentially.
- If using `make -j`, always specify the job count, for example
  `make -j$(nproc)`.
- For Rust workspace commands on the host, use
  `--manifest-path src/rust/Cargo.toml`.
- If `messages/*.proto` changes, run
  `./scripts/dev_exec.sh make generate-protobufs` before direct Rust `cargo`
  commands.
- Do not stop the Rust docker container unless you restart it, for example after
  `.containerversion` changes.

## Done Means

- The relevant detailed `doc/agent/*.md` guidance has been followed.
- Formatting has been run for modified languages when practical.
- Relevant tests or checks have been run, or the final response clearly states
  why they were not run.
- The final response summarizes changed files and verification.
