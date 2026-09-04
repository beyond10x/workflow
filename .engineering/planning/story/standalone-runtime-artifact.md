---
format: aep.planning-md/1
id: story:standalone-runtime-artifact
kind: story
status: active
title: Publish Workflow as an ESS component release
summary: Regenerate Workflow as a runnable, independently versioned OCI image and Helm release bundle.
relations:
- derived_from: epic:authenticated-workflow-surface
- serves: vision:O4
scope:
- confidence: cited
  path: .dockerignore
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
  path: Taskfile.yml
- confidence: cited
  path: generated
- confidence: cited
  path: service.yaml
revision: 9
---
# Story: Publish Workflow as an ESS component release

## Outcome

Workflow is generated from Service SDK 0.5, built and published by its own repository as immutable OCI runtime and chart release units, and consumable by downstream ESS stack locks without rebuilding Devcenter.

## Context

Service SDK now owns the reusable process host and generates the ESS component, realization, build, runtime, BuildKit, and Helm artifacts. Workflow should declare only its release coordinates and generated-service package; it must not maintain a parallel handwritten host or Dockerfile.

## Acceptance

- Workflow pins the released Service SDK generator and commits its complete drift-checked output.
- The generated Rust binary starts with Identity authentication, SQLite Eventlog persistence, health probes, and graceful shutdown.
- ESS validates the component/build/runtime chain and Helm lints the generated configuration-neutral chart.
- Workflow CI builds the generated BuildKit graph and publishes digest-addressed runtime, chart, evidence, and component bundle artifacts from a version tag.
- A smoke test proves process startup and durable restart behavior without a handwritten runtime crate.
- Version and release notes identify the independently deployable Workflow release.

## Out of Scope

Changing Workflow operations, adding execution semantics, or placing environment-specific values in this public repository.

## Open Questions

None.
