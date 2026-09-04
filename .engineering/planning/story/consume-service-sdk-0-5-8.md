---
format: aep.planning-md/1
id: story:consume-service-sdk-0-5-8
kind: story
status: implemented
title: Consume Service SDK 0.5.8
summary: Regenerate Workflow with optional projection-field correctness.
relations:
- derived_from: epic:authenticated-workflow-surface
- serves: vision:O4
- serves: vision:O5
scope:
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Cargo.toml
- confidence: cited
  path: generated
- confidence: cited
  path: service.yaml
- confidence: cited
  path: service/scenarios/workflow.yaml
revision: 5
---
## Outcome

Workflow reads populated definition projections successfully because its generated realization plan carries the Service SDK 0.5.8 optional-field contract.

## Acceptance

Workflow pins the exact released Service SDK 0.5.8 source commit, regenerates a `service-realization-plan/3`, passes the complete repository gate, and a create-then-list regression returns the created workflow instead of `500 service_contract`.

## Scope

`service.yaml`, `Cargo.toml`, `Cargo.lock`, the deterministic `generated/` tree, release notes, and the populated-library regression.
