---
format: aep.planning-md/1
id: story:starter-library-resource-bundle
kind: story
status: active
title: Reconcile the starter Workflow library
summary: Materialize reusable starter definitions from a service-owned immutable resource on startup.
relations:
- derived_from: epic:authenticated-workflow-surface
- serves: vision:O4
- serves: vision:O5
scope:
- confidence: cited
  path: generated/conformance
- confidence: cited
  path: generated/runtime
- confidence: cited
  path: generated/rust
- confidence: cited
  path: service.yaml
- confidence: cited
  path: service/runtime.yaml
- confidence: cited
  path: spec/workflow
revision: 5
---
## Context

A fresh Workflow deployment contains no definitions, so its healthy HTTP process still presents an unusable library. Devcenter currently compensates with a user-triggered installer, which makes product readiness depend on a privileged browser action and duplicates Workflow's write model outside the owning service.

## Acceptance

Starting Workflow against an empty durable store idempotently reconciles the service-owned Code review, Security review, and Reverse AEP + ESS definitions with published active immutable revisions, while a restart creates no duplicate aggregate or revision and requires no direct database seed or downstream product command sequence.

## Scope

The confirmed unit changed no Workflow product source. A focused `service.yaml` probe established that the pinned Service SDK 0.5.7 rejects a `resources` declaration; the probe was reverted. The durable changes are this story, `dependency-blocker:service-sdk-resource-reconciliation`, and their journal entries.

The originally inferred runtime, generated projection, immutable resource, and conformance-test paths remain future scope after Service SDK supplies a validated resource input and idempotent pre-readiness reconciliation contract.
