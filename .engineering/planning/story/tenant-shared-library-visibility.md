---
format: aep.planning-md/1
id: story:tenant-shared-library-visibility
kind: story
status: implemented
title: Expose reusable workflows without granting ownership
summary: Let admitted engineers read tenant-shared definitions while writes stay owner-bound.
relations:
- derived_from: epic:authenticated-workflow-surface
- serves: vision:O5
scope:
- confidence: cited
  path: .github/workflows/release.yml
- confidence: cited
  path: CHANGELOG.md
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Cargo.toml
- confidence: cited
  path: README.md
- confidence: cited
  path: generated
- confidence: cited
  path: service.yaml
- confidence: cited
  path: service/runtime.yaml
- confidence: cited
  path: service/scenarios
revision: 6
---
## Outcome

Authorized engineers can read reusable Workflow definitions published by another owner in the same Identity-derived tenant and exact optional realm, while only the owner can revise or activate them.

## Context

The standalone library currently filters reads through aggregate ownership, so deployment- or operator-published definitions are invisible to everyone else. Service SDK 0.5.7 introduces scopes-only projection visibility without weakening mutation authorization.

## Acceptance

- Pin the released Service SDK revision that implements scopes-only projection visibility.
- Workflow projections and queries filter by conjunctive scope facts without requiring reader ownership.
- Authoring, publishing, and activation remain owner-and-scopes guarded.
- A conformance scenario proves a second authority in the same tenant can read a tenant-visible published workflow; the pinned SDK test suite proves that this visibility does not satisfy owner-bound mutation authorization.
- Regenerate the complete service tree and publish an independently deployable Workflow release.

## Out of Scope

Executing graphs, project-bound workflow admission, deployment-specific identities, and the Devcenter starter definition bundle.

## Scope

- cited: service.yaml, service/runtime.yaml, service/scenarios, generated, Cargo manifests, README, changelog, and release metadata.
