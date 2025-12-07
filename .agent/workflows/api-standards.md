---
description: API design standards and best practices
---

# API Design Standards

**AI AGENT RULE: Always follow RESTful API best practices when creating or modifying endpoints.**

## Core Principles

### 1. RESTful Design
- Use HTTP methods correctly (GET, POST, PUT, DELETE, PATCH)
- Use nouns for resources, not verbs
- Use plural nouns for collections
- Use hierarchical structure for relationships

### 2. Versioning
- **Always version APIs**: `/api/v1/`, `/api/v2/`
- Maintain backward compatibility
- Deprecate old versions gracefully

### 3. Naming Conventions
- Use lowercase with hyphens: `/api/v1/mining-targets`
- Be consistent across all endpoints
- Use clear, descriptive names

### 4. Response Format
- Always return JSON
- Use consistent structure
- Include proper HTTP status codes

## URL Structure

### Good Examples ✅
```
GET    /api/v1/targets              # List all targets
GET    /api/v1/targets/{id}         # Get specific target
GET    /api/v1/targets/current      # Get current target
POST   /api/v1/targets              # Create target
PUT    /api/v1/targets/{id}         # Update target
DELETE /api/v1/targets/{id}         # Delete target
```

### Bad Examples ❌
```
GET    /getTargets                  # No versioning, verb in URL
GET    /api/target                  # Singular noun
GET    /api/v1/get-current-target   # Verb in URL
POST   /api/v1/createTarget         # Verb in URL, camelCase
```

## HTTP Methods

| Method | Purpose | Example |
|--------|---------|---------|
| GET | Retrieve resource(s) | `GET /api/v1/targets` |
| POST | Create new resource | `POST /api/v1/targets` |
| PUT | Update entire resource | `PUT /api/v1/targets/123` |
| PATCH | Partial update | `PATCH /api/v1/targets/123` |
| DELETE | Delete resource | `DELETE /api/v1/targets/123` |

## HTTP Status Codes

Use appropriate status codes:

| Code | Meaning | When to Use |
|------|---------|-------------|
| 200 | OK | Successful GET, PUT, PATCH, DELETE |
| 201 | Created | Successful POST |
| 204 | No Content | Successful DELETE with no response body |
| 400 | Bad Request | Invalid request data |
| 401 | Unauthorized | Authentication required |
| 403 | Forbidden | Authenticated but not authorized |
| 404 | Not Found | Resource doesn't exist |
| 409 | Conflict | Resource conflict (e.g., duplicate) |
| 500 | Internal Server Error | Server error |

## Response Structure

### Success Response
```json
{
  "data": {
    "id": "123",
    "name": "supportxmr",
    "type": "pool"
  }
}
```

### Error Response
```json
{
  "error": {
    "code": "INVALID_TARGET",
    "message": "Target not found",
    "details": "No target with ID 123 exists"
  }
}
```

### Collection Response
```json
{
  "data": [
    {"id": "1", "name": "target1"},
    {"id": "2", "name": "target2"}
  ],
  "meta": {
    "total": 2,
    "page": 1,
    "per_page": 10
  }
}
```

## Pagination

For large collections:
```
GET /api/v1/targets?page=2&per_page=20
```

Response:
```json
{
  "data": [...],
  "meta": {
    "total": 100,
    "page": 2,
    "per_page": 20,
    "total_pages": 5
  },
  "links": {
    "first": "/api/v1/targets?page=1",
    "prev": "/api/v1/targets?page=1",
    "next": "/api/v1/targets?page=3",
    "last": "/api/v1/targets?page=5"
  }
}
```

## Filtering & Sorting

```
GET /api/v1/targets?coin=XMR&sort=-profitability
GET /api/v1/targets?type=pool&algorithm=RandomX
```

## Documentation

### OpenAPI/Swagger
- Document all endpoints
- Include request/response examples
- Specify required vs optional fields

### Endpoint Documentation Template
```rust
/// GET /api/v1/targets - List all mining targets
///
/// Returns a list of all configured mining targets with their profitability scores.
///
/// # Query Parameters
/// - `coin` (optional): Filter by coin (e.g., "XMR")
/// - `type` (optional): Filter by type ("pool" or "daemon")
///
/// # Response
/// - 200: Success
/// - 500: Internal server error
pub async fn list_targets(
    State(state): State<AppState>,
    Query(params): Query<TargetFilters>,
) -> Result<Json<Vec<ProfitabilityScore>>, ApiError> {
    // Implementation
}
```

## Security

- **Authentication**: Use JWT or API keys
- **Rate Limiting**: Prevent abuse
- **CORS**: Configure properly
- **Input Validation**: Validate all inputs
- **SQL Injection**: Use parameterized queries

## Checklist

Before deploying an API:
- [ ] Versioned (`/api/v1/`)
- [ ] RESTful naming (nouns, not verbs)
- [ ] Proper HTTP methods
- [ ] Correct status codes
- [ ] Consistent response format
- [ ] Error handling
- [ ] Documentation
- [ ] Input validation
- [ ] Authentication (if needed)
- [ ] Rate limiting (if needed)

## DefPool API Structure

```
/api/v1/target              # GET - Current mining target
/api/v1/targets             # GET - List all targets
/api/v1/targets/current     # GET - Current target name
/api/v1/targets/{id}        # GET - Specific target (future)
/api/v1/profitability       # GET - Profitability scores (future)
/api/v1/stats               # GET - System stats (future)
```

## Legacy Support

Maintain backward compatibility:
```rust
// New versioned route
.route("/api/v1/target", get(api::get_current_target))
// Legacy route (deprecated)
.route("/target", get(api::get_current_target))
```

Mark legacy routes as deprecated in documentation.
