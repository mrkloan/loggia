## 1. Domain Model

- [ ] 1.1 Create `domain/src/models/user.rs` with a private `username` field, `User::new(username: String) -> DomainResult<Self>` constructor (trim + blank check), and `username()` accessor
- [ ] 1.2 Derive `serde::Serialize` on `User` so it can be serialized directly as a JSON response
- [ ] 1.3 Export `user` module from `domain/src/models/mod.rs`

## 2. API Extractor

- [ ] 2.1 Create `application/api/src/http/extractors/mod.rs` and `user.rs` implementing `FromRequestParts` for `User`
- [ ] 2.2 Return `401 Unauthorized` with `{ "error": "Missing X-Vouch-User header" }` when the `X-Vouch-User` header is absent
- [ ] 2.3 Return `403 Forbidden` with `{ "error": "<DomainError display string>" }` when `User::new` returns a `DomainError::Validation`

## 3. GET /me Endpoint

- [ ] 3.1 Create `application/api/src/http/me.rs` with a handler that extracts `User` and returns `200 OK` with the serialized user
- [ ] 3.2 Register `GET /me` in `application/api/src/http/mod.rs`

## 4. Verification

- [ ] 4.1 Build the project and confirm zero compile errors
- [ ] 4.2 Manually verify `GET /me` returns `{ "username": "alice" }` with header `X-Vouch-User: alice`
- [ ] 4.3 Manually verify `GET /me` returns `401` with no header
- [ ] 4.4 Manually verify `GET /me` returns `403` with a blank `X-Vouch-User` header value
- [ ] 4.5 Manually verify `GET /health` still responds without the `X-Vouch-User` header
