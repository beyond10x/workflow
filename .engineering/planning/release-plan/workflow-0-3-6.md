---
format: aep.planning-md/1
id: release-plan:workflow-0-3-6
kind: release-plan
status: implemented
title: Release Workflow 0.3.6
summary: Publish Workflow with valid populated-library projections.
relations:
- delivers: story:consume-service-sdk-0-5-8
revision: 3
---
## Outcome

Workflow 0.3.6 is published from its exact bot-authored main commit with independently built ESS component, runtime image, chart, evidence, and bundle artifacts.

## Scope

Version alignment, deterministic Service SDK regeneration, complete Workflow gate, bot-authored main publication, annotated tag, native Workflow release, and Devcenter runtime promotion.

## Qualification

The populated create-then-list path passes locally and in the repository gate. Both release workflows bind their artifacts to the exact tagged source commit.
