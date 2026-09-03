# Changelog

All notable changes to Workflow are documented here.

## 0.1.0 - 2026-09-03

- Release the transport-neutral `workflow-domain` crate boundary.
- Provide validated, borrowable `WorkflowId` values with an explicit error for empty input.
- Keep HTTP, persistence, triggers, durable runs, leases, cancellation, execution, and evidence
  outside this release; those capabilities remain planned.
