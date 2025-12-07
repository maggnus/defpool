---
description: Commit and push changes after completing a stage
---

# Git Commit Workflow

After completing any major stage of work (e.g., implementing a feature, fixing a bug, completing verification), commit and push changes:

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

## When to commit:
- After implementing a new feature
- After fixing a critical bug
- After completing verification of changes
- After updating documentation
- At the end of a task boundary

## Commit message format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation
- `refactor:` for code refactoring
- `test:` for adding tests
- `chore:` for maintenance tasks
