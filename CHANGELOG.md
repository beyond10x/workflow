# Changelog

## 0.3.4 - 2026-09-04

- Regenerate the runnable Workflow service with Service SDK 0.5.6 and Connectors 0.5.11, and use
  ESS 0.13.5's component-owned release toolchains so the repository publishes its runtime, chart,
  evidence, and component bundle without a Devcenter rebuild.

## 0.3.3 - 2026-09-04

- Pin ESS 0.13.4 so the generic component action executes portably from a clean checkout.

## 0.3.2 - 2026-09-04

- Publish through the generic ESS 0.13.3 action, with robust Helm digest capture and no release
  dependency on Service SDK internals.

## 0.3.1 - 2026-09-04

- Invoke the reusable Service SDK component-release action and keep the component gate portable on
  a clean GitHub runner.

## 0.3.0 - 2026-09-04

- Generate the executable Identity HTTP host and durable SQLite lifecycle from Service SDK 0.5.0.
- Publish canonical ESS component, realization, build, runtime, BuildKit, and Helm inputs for an
  independently deployable Workflow release.
- Keep every runtime and release input under the generated-tree drift gate; Workflow carries no
  handwritten host or Dockerfile.

## 0.2.0 - 2026-09-03

- Replace the ID-only 0.1.0 crate with a definition-only Service SDK package and generated Rust
  service, official `WorkflowClient`, Identity HTTP boundary, OpenAPI, Connector contribution, and
  conformance scenarios.
- Add mutable owned draft DAGs with guarded node/edge mutation, deterministic validation, canonical
  immutable revision snapshots and digests, and owned revision activation.
- Require the exact `urn:b10x:workflow` audience, `workflows.read` and `workflows.manage` scopes,
  optimistic concurrency, and idempotency with no predecessor or 0.1.0 compatibility layer.

All notable changes to Workflow are documented here.

## 0.1.0 - 2026-09-03

- Release the transport-neutral `workflow-domain` crate boundary.
- Provide validated, borrowable `WorkflowId` values with an explicit error for empty input.
- Keep HTTP, persistence, triggers, durable runs, leases, cancellation, execution, and evidence
  outside this release; those capabilities remain planned.
