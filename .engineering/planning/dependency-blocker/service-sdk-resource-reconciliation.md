---
format: aep.planning-md/1
id: dependency-blocker:service-sdk-resource-reconciliation
kind: dependency-blocker
status: open
title: Service SDK lacks immutable resource reconciliation
summary: Workflow starter resources need an SDK-owned, tenant-neutral pre-readiness reconciliation contract.
owner: workflow
relations:
- blocks: story:starter-library-resource-bundle
withholds: test_result
revision: 1
---
## Missing contract

Released Service SDK 0.5.7 cannot declare or reconcile an immutable service-owned resource bundle. Its `service/1` manifest rejects `resources`, `service-definition/3` has no resource obligation, and the generated host has no pre-readiness reconciliation hook.

Workflow storage is tenant-partitioned. A Workflow-local seed would therefore have to embed or invent a tenant and would create a bespoke deployment path, violating the service boundary and repository invariants.

## Evidence

A focused red probe added `resources: [service/resources/starter-library.yaml]` to `service.yaml`. The pinned builder exited 1 with `unknown field 'resources'` and listed the supported fields. The probe was then reverted. Existing formatting, clippy, tests, and deterministic generated-output checks pass.

## Required upstream capability

Add to Service SDK a validated package resource input, canonical embedded-resource IR, an SDK-owned idempotent pre-readiness reconciler with stable resource identity and digest, service-owned tenant-neutral visibility, and conformance coverage that initializes the same store twice without changing aggregate or revision counts.
