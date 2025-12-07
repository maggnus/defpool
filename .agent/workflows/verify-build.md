---
description: Verify build with no warnings before committing
---

# Build Verification Workflow

Before committing code, ensure the build is clean with no warnings.

## Steps

1. Build the server
```bash
cd defpool-server
cargo build --release 2>&1 | tee build.log
```

2. Check for warnings
```bash
if grep -q "warning:" build.log; then
    echo "❌ Build has warnings. Fix them before committing."
    grep "warning:" build.log
    exit 1
else
    echo "✅ Build is clean"
fi
```

3. Build the proxy
```bash
cd ../defpool-proxy
cargo build --release 2>&1 | tee build.log
```

4. Check for warnings
```bash
if grep -q "warning:" build.log; then
    echo "❌ Build has warnings. Fix them before committing."
    grep "warning:" build.log
    exit 1
else
    echo "✅ Build is clean"
fi
```

5. Run tests (if any)
```bash
cargo test
```

## Quick Command
```bash
# From project root
cd defpool-server && cargo build --release --quiet && \
cd ../defpool-proxy && cargo build --release --quiet && \
echo "✅ All builds clean"
```

## CI/CD Integration
Add to `.github/workflows/build.yml`:
```yaml
- name: Build with warnings as errors
  run: |
    cd defpool-server && cargo build --release -- -D warnings
    cd ../defpool-proxy && cargo build --release -- -D warnings
```
