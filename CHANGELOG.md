# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] — 2026-04-05

### Added
- `kvlar-proxy`: Full SHIELD cloud integration — real-time policy evaluation via `POST /v1/evaluate`
- `kvlar-proxy`: RADAR audit streaming — batch event ingest via `POST /v1/events`
- `kvlar-proxy`: Human-in-the-loop escalation workflow — creates escalations on SHIELD, polls for approval/denial
- `kvlar-proxy`: Configurable escalation polling interval and timeout (default 5s / 300s)
- `kvlar-proxy`: `kvlar_cloud_url` and `kvlar_api_key` fields in `ProxyConfig` for cloud mode
- `kvlar-proxy`: `ShieldClient` — async HTTP client for SHIELD API with Bearer auth
- `kvlar-proxy`: `RadarClient` — async HTTP client for RADAR event ingest
- `kvlar-proxy`: `EscalationClient` — escalation lifecycle management (create, poll, resolve)
- `kvlar-cli`: `--api-key` and `--agent-id` flags on `kvlar wrap` for cloud mode configuration
- `kvlar-cli`: `kvlar init --cloud` prints step-by-step SHIELD onboarding instructions
- `kvlar-cli`: `kvlar validate --cloud` flag accepted (future feature, documented in `--help`)
- README.md: Cloud Mode (SHIELD) quickstart section with example config and feature comparison table
- 156 total unit tests across all crates (up from 23 in v0.3.0)

### Changed
- `kvlar-proxy`: Handler now routes policy decisions through SHIELD when cloud config is present
- `kvlar-proxy`: Deny responses include `_kvlar.code=POLICY_DENY` in JSON-RPC result metadata
- `kvlar-proxy`: Timeout on escalation treated as deny (fail-closed)

### Fixed
- Escalation `"expired"` server-side status now correctly triggers `ApprovalError::Timeout`

## [0.3.0] — Initial Release

### Added
- `kvlar-core`: Policy engine with YAML-based policy definitions
- `kvlar-core`: Action, Decision, Policy, Rule, and Engine types
- `kvlar-core`: Fail-closed default (deny when no rule matches)
- `kvlar-core`: Regex-based parameter matching in rules
- `kvlar-audit`: Structured audit event logging
- `kvlar-audit`: JSON and human-readable output formats
- `kvlar-proxy`: MCP proxy configuration and scaffolding
- `kvlar-cli`: `validate`, `evaluate`, and `inspect` commands
- 23 unit tests across all crates
- CI workflow with check, test, clippy, format, and doc jobs
- Apache 2.0 license
