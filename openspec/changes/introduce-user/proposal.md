## Why

The application has no concept of identity. Introducing a `User` entity establishes the foundation for attributing actions to specific users, with the `username` acting as a future anchor for linking database rows to real users — without the overhead of storing users in the database at this stage.

## What Changes

- Introduce a transient `User` domain model with a `username` string attribute, enforced as a valid non-blank value via invariant validation
- Require the `X-Vouch-User` HTTP header on all protected API requests; absent header returns 401, invalid value returns 403
- Expose a `GET /me` endpoint that reflects the authenticated user back as a serialized `User` entity
- The `/health` endpoint remains public and exempt from authentication

## Capabilities

### New Capabilities

- `user-identity`: Transient `User` domain model with invariant validation and extraction from the `X-Vouch-User` HTTP header
- `current-user-api`: `GET /me` endpoint returning the authenticated user's identity

### Modified Capabilities

## Impact

- `domain`: New `User` model in `domain/src/models/`
- `application/api`: New Axum extractor for `User`; router split into public/protected; new `GET /me` handler
- No database changes; no new dependencies required
