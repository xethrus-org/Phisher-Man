# Development Guide

Quick guide on how we work on PhisherMan. Keep things clean and traceable.

## Branching

### Branch naming

- **`main`** - production code, keep it stable
- **`feature/feature-name`** - new features
  - ex: `feature/gpt-integration`, `feature/email-templates`
- **`fix/bug-description`** - bug fixes
  - ex: `fix/email-validation`, `fix/database-connection`
- **`refactor/what-changed`** - refactoring without breaking stuff
  - ex: `refactor/error-handling`, `refactor/database-models`
- **`docs/what-changed`** - documentation updates
- **`chore/task-description`** - maintenance, deps, etc.

### Workflow

```bash
# start new work
git checkout main
git pull origin main
git checkout -b feature/your-feature-name

# do your thing, commit as you go

# push and PR when ready
git push -u origin feature/your-feature-name
```

## Commit Format

Using **Conventional Commits** to keep history readable.

### Structure

```
type(scope): brief description

- optional longer explanation
- why you made the change
- any important context
```

### Types

- **`feat`** - new feature
- **`fix`** - bug fix
- **`refactor`** - code cleanup, no functional changes
- **`docs`** - documentation
- **`test`** - tests
- **`chore`** - maintenance (deps, config, etc.)
- **`perf`** - performance improvements
- **`style`** - formatting, whitespace

### Scope (optional but helpful)

What part of the codebase:
- `api`, `db`, `models`, `handlers`, `services`, `frontend`, `config`

### Examples

```bash
feat(api): add campaign creation endpoint

fix(db): resolve connection pool timeout

refactor(handlers): simplify error handling

docs: update API documentation

chore(deps): update axum to 0.7.5
```

### Keep in mind

- Small focused commits > giant commits
- Present tense: "add" not "added"
- Explain *why* in the body if it's not obvious
- The diff shows *what* changed, your message explains *why*

## Code Organization

```
src/
├── main.rs           # entry point
├── config.rs         # config management
├── error.rs          # error types
├── models/           # database models
├── handlers/         # http request handlers
├── services/         # business logic
└── db/               # database stuff

migrations/           # sql migrations
static/               # frontend (html/css/js)
tests/                # integration tests
```

### Style

- snake_case for files: `email_sender.rs`
- run `cargo fmt` before committing
- run `cargo clippy` to catch issues
- keep functions small and focused
- comment complex logic

## Testing

```bash
cargo test                    # run all tests
cargo test -- --nocapture     # see println output
cargo test test_name          # run specific test
```

## Database Migrations

Create numbered SQL files in `migrations/`:
```
migrations/002_add_user_auth.sql
```

Run migrations:
```bash
sqlx migrate run
sqlx migrate revert  # undo last one
```

## Environment Setup

1. Copy `.env.example` to `.env`
2. Fill in your database URL and other settings
3. Don't commit `.env` (already in `.gitignore`)

## Questions?

When in doubt, ask. Keeping things consistent beats getting creative with conventions.
