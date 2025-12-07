---
description: Coding standards and best practices for DefPool
---

# DefPool Coding Standards

## Core Principles

### 1. Avoid Code Duplication (DRY)
- Extract common logic into reusable functions/modules
- Use traits for shared behavior
- Create utility modules for repeated patterns

### 2. Clean Code
- Descriptive variable and function names
- Single Responsibility Principle (SRP)
- Keep functions small and focused
- Clear error handling with context

### 3. Modularization
- Separate concerns into distinct modules
- Each module should have a clear purpose
- Use `mod.rs` to organize module exports
- Keep related functionality together

### 4. Scalability & Replaceability
- **Use Abstract Interfaces**: Define traits for all major components
- **Dependency Injection**: Pass dependencies rather than hardcoding
- **Configuration-Driven**: Make behavior configurable
- **Plugin Architecture**: Components should be swappable

### 5. Code Cleanup & Optimization
- **Remove Dead Code**: Delete unused functions, structs, and imports
- **Remove Commented Code**: Use git history instead
- **Optimize Imports**: Only import what's needed
- **Reusability**: Extract repeated logic into helper functions
- **Performance**: Use appropriate data structures
- **Memory**: Avoid unnecessary clones, use references where possible

## Architecture Patterns

### Trait-Based Design
```rust
// ✅ Good: Abstract interface
pub trait PriceProvider {
    async fn get_price(&self, coin: &str) -> Result<f64>;
}

// ✅ Good: Concrete implementation
pub struct CoinGeckoProvider { /* ... */ }
impl PriceProvider for CoinGeckoProvider { /* ... */ }

// ✅ Good: Easy to swap providers
pub struct ProfitCalculator<P: PriceProvider> {
    price_provider: P,
}
```

### Module Organization
```
defpool-server/
├── src/
│   ├── main.rs           # Entry point
│   ├── config.rs         # Configuration
│   ├── api/              # API layer
│   │   ├── mod.rs
│   │   ├── routes.rs
│   │   └── handlers.rs
│   ├── profitability/    # Business logic
│   │   ├── mod.rs
│   │   ├── calculator.rs
│   │   └── providers/
│   │       ├── mod.rs
│   │       ├── price.rs
│   │       └── difficulty.rs
│   └── state.rs          # Shared state
```

### Error Handling
```rust
// ✅ Good: Custom error types with context
#[derive(Debug, thiserror::Error)]
pub enum ProfitabilityError {
    #[error("Failed to fetch price for {coin}: {source}")]
    PriceFetchError { coin: String, source: reqwest::Error },
}
```

## Code Cleanup Checklist
Before committing:
- [ ] Remove all unused imports
- [ ] Remove all unused functions/structs
- [ ] Remove all commented-out code
- [ ] Remove all `#[allow(dead_code)]` if code is truly unused
- [ ] Extract repeated logic into helper functions
- [ ] Optimize data structures (Vec vs HashMap, etc.)
- [ ] Use references instead of clones where possible
- [ ] Run `cargo clippy` and fix all warnings

## Code Review Checklist
- [ ] No duplicated code
- [ ] Functions are small (<50 lines)
- [ ] Clear separation of concerns
- [ ] Traits used for abstractions
- [ ] Dependencies are injected
- [ ] Error handling with context
- [ ] Tests for critical paths
- [ ] No dead code
- [ ] No unused imports
- [ ] Zero compiler warnings

