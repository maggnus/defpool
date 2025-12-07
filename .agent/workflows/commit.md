---
description: Commit and push changes after completing a stage
---

# Git Commit Workflow

**AI AGENT RULE: Automatically commit and push after completing any major stage of work.**

After completing any major stage of work (e.g., implementing a feature, fixing a bug, completing verification), commit and push changes:

## When to Commit

**Always commit after:**
- ✅ Implementing a new feature
- ✅ Fixing a critical bug
- ✅ Completing verification of changes
- ✅ Updating documentation
- ✅ At the end of a task boundary
- ✅ Before switching to a new task
- ✅ After achieving zero warnings build

**Do NOT commit:**
- ❌ Broken/non-compiling code
- ❌ Code with failing tests
- ❌ Code with compiler warnings

## Steps

1. Check git status
```bash
git status
```

2. Add all changes
```bash
git add .
```

3. Create a descriptive commit message based on what was accomplished
```bash
git commit -m "feat: [brief description of the stage completed]"
```

4. Push to remote
```bash
git push
```

## Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation
- `refactor:` for code refactoring
- `test:` for adding tests
- `chore:` for maintenance tasks
- `perf:` for performance improvements

### Examples

```bash
# Good commit messages
git commit -m "feat: add multi-pool profitability calculation"
git commit -m "fix: resolve memory leak in connection handler"
git commit -m "docs: update API documentation with new endpoints"
git commit -m "refactor: extract price provider into trait"
git commit -m "chore: update dependencies to latest versions"

# Bad commit messages
git commit -m "update"
git commit -m "fix stuff"
git commit -m "wip"
```

## AI Agent Automation

**The AI agent should:**
1. ✅ Run `/verify-build` before committing
2. ✅ Ensure zero warnings
3. ✅ Update CHANGELOG.md for significant changes
4. ✅ Use descriptive commit messages
5. ✅ Push immediately after committing
6. ✅ Commit at logical breakpoints (end of features, not mid-implementation)

**Example workflow:**
```
1. Implement feature
2. Run cargo build --release (verify no warnings)
3. Update CHANGELOG.md
4. git add .
5. git commit -m "feat: implement feature X"
6. git push
```

