---
format: aep.planning-md/1
id: story:workflow-domain-0-1-0-release
kind: story
status: implemented
title: Release Workflow domain boundary 0.1.0
summary: Publish the implemented minimal domain boundary without overstating planned service capabilities.
relations:
- derived_from: initiative:standalone-workflow-service
- serves: vision:O5
scope:
- confidence: inferred
  path: CHANGELOG.md
revision: 5
---
# Release Workflow domain boundary 0.1.0

## Context

Workflow currently owns a deliberately small, transport-neutral Rust domain boundary. This release must name only that implemented surface and must not imply that the planned HTTP, persistence, trigger, lease, or execution services exist.

## Acceptance

The 0.1.0 tag points to the tested public commit whose changelog explicitly limits the release to WorkflowId validation and the transport-neutral domain crate.

## Scope

Release notes and immutable release evidence for the existing workflow-domain crate.
