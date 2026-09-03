<!--
  generated from workflow v1
  model digest 6a79078047815754e0dda89f285e08689037af0adfd4277dfcc305f1d003df1b
  contract digest 1f20a14314130c4a1d048f7a1f6cb2319a04e70ea589b6d3b51b7e4443f39b95
  do not edit: regenerate with `ess synthesize`
-->
# Synthesis plan — workflow v1

Scope: `component-skeletons`, planned by `ess-synth`. Regenerate with `ess synthesize`.

82 capabilities: **64 generated**, **17 obligations**, **1 refused**. An obligation is yours to implement against its contract; a refusal is a fact about this synthesis scope, not about the specification.

## Generated

| capability | source |
| --- | --- |
| domain type | `workflow.definition.CallNode` |
| domain type | `workflow.definition.CompleteNode` |
| domain type | `workflow.definition.ComputeNode` |
| domain type | `workflow.definition.DraftEdge.State` |
| domain type | `workflow.definition.DraftId` |
| domain type | `workflow.definition.DraftNode.State` |
| domain type | `workflow.definition.DraftRow` |
| domain type | `workflow.definition.EdgeId` |
| domain type | `workflow.definition.EdgeSnapshot` |
| domain type | `workflow.definition.EmitNode` |
| domain type | `workflow.definition.InvokeNode` |
| domain type | `workflow.definition.JudgeNode` |
| domain type | `workflow.definition.NodeDefinition` |
| domain type | `workflow.definition.NodeId` |
| domain type | `workflow.definition.NodeSnapshot` |
| domain type | `workflow.definition.OwnerRef` |
| domain type | `workflow.definition.PrincipalRef` |
| domain type | `workflow.definition.ProjectRef` |
| domain type | `workflow.definition.ReadNode` |
| domain type | `workflow.definition.RevisionId` |
| domain type | `workflow.definition.RevisionRow` |
| domain type | `workflow.definition.ScopeBindings` |
| domain type | `workflow.definition.TeamRef` |
| domain type | `workflow.definition.WaitNode` |
| domain type | `workflow.definition.Workflow.State` |
| domain type | `workflow.definition.WorkflowDraft.State` |
| domain type | `workflow.definition.WorkflowId` |
| domain type | `workflow.definition.WorkflowRevision.State` |
| domain type | `workflow.definition.WorkflowRow` |
| entity lifecycle | `workflow.definition.DraftEdge` |
| entity lifecycle | `workflow.definition.DraftNode` |
| entity lifecycle | `workflow.definition.Workflow` |
| entity lifecycle | `workflow.definition.WorkflowDraft` |
| entity lifecycle | `workflow.definition.WorkflowRevision` |
| command contract | `workflow.definition.ActivateRevision` |
| command contract | `workflow.definition.AddNode` |
| command contract | `workflow.definition.ConnectNodes` |
| command contract | `workflow.definition.CreateDraft` |
| command contract | `workflow.definition.CreateWorkflow` |
| command contract | `workflow.definition.DiscardDraft` |
| command contract | `workflow.definition.DisconnectNodes` |
| command contract | `workflow.definition.PublishDraft` |
| command contract | `workflow.definition.RemoveNode` |
| command contract | `workflow.definition.ReplaceNode` |
| command contract | `workflow.definition.ValidateDraft` |
| event type | `workflow.definition.DraftCreated` |
| event type | `workflow.definition.DraftDiscarded` |
| event type | `workflow.definition.DraftValidated` |
| event type | `workflow.definition.NodeAdded` |
| event type | `workflow.definition.NodeRemoved` |
| event type | `workflow.definition.NodeReplaced` |
| event type | `workflow.definition.NodesConnected` |
| event type | `workflow.definition.NodesDisconnected` |
| event type | `workflow.definition.RevisionActivated` |
| event type | `workflow.definition.RevisionPublished` |
| event type | `workflow.definition.WorkflowCreated` |
| view type | `workflow.definition.DraftById` |
| view type | `workflow.definition.Drafts` |
| view type | `workflow.definition.RevisionById` |
| view type | `workflow.definition.Revisions` |
| view type | `workflow.definition.WorkflowById` |
| view type | `workflow.definition.Workflows` |
| component port | `workflow-service` |
| component transport | `workflow-service` |

## Obligations — yours to implement

| capability | source | why not generated | contract |
| --- | --- | --- | --- |
| command behaviour | `workflow.definition.ActivateRevision` | the contract is declared; the algorithm is not | given `workflow.definition.ActivateRevision` input, decide and enact exactly one outcome — `activated` otherwise, updates `workflow.definition.Workflow`, emits `workflow.definition.RevisionActivated` |
| command behaviour | `workflow.definition.AddNode` | the contract is declared; the algorithm is not | given `workflow.definition.AddNode` input, decide and enact exactly one outcome — `added` otherwise, creates `workflow.definition.DraftNode`, emits `workflow.definition.NodeAdded` |
| command behaviour | `workflow.definition.ConnectNodes` | the contract is declared; the algorithm is not | given `workflow.definition.ConnectNodes` input, decide and enact exactly one outcome — `connected` otherwise, creates `workflow.definition.DraftEdge`, emits `workflow.definition.NodesConnected` |
| command behaviour | `workflow.definition.CreateDraft` | the contract is declared; the algorithm is not | given `workflow.definition.CreateDraft` input, decide and enact exactly one outcome — `created` otherwise, creates `workflow.definition.WorkflowDraft`, emits `workflow.definition.DraftCreated` |
| command behaviour | `workflow.definition.CreateWorkflow` | the contract is declared; the algorithm is not | given `workflow.definition.CreateWorkflow` input, decide and enact exactly one outcome — `created` otherwise, creates `workflow.definition.Workflow`, emits `workflow.definition.WorkflowCreated` |
| command behaviour | `workflow.definition.DiscardDraft` | the contract is declared; the algorithm is not | given `workflow.definition.DiscardDraft` input, decide and enact exactly one outcome — `discarded` otherwise, takes `discard` of `workflow.definition.WorkflowDraft`, emits `workflow.definition.DraftDiscarded` |
| command behaviour | `workflow.definition.DisconnectNodes` | the contract is declared; the algorithm is not | given `workflow.definition.DisconnectNodes` input, decide and enact exactly one outcome — `disconnected` otherwise, takes `disconnect` of `workflow.definition.DraftEdge`, emits `workflow.definition.NodesDisconnected` |
| command behaviour | `workflow.definition.PublishDraft` | the contract is declared; the algorithm is not | given `workflow.definition.PublishDraft` input, decide and enact exactly one outcome — `published` otherwise, creates `workflow.definition.WorkflowRevision`, emits `workflow.definition.RevisionPublished` |
| command behaviour | `workflow.definition.RemoveNode` | the contract is declared; the algorithm is not | given `workflow.definition.RemoveNode` input, decide and enact exactly one outcome — `removed` otherwise, takes `remove` of `workflow.definition.DraftNode`, emits `workflow.definition.NodeRemoved` |
| command behaviour | `workflow.definition.ReplaceNode` | the contract is declared; the algorithm is not | given `workflow.definition.ReplaceNode` input, decide and enact exactly one outcome — `replaced` otherwise, updates `workflow.definition.DraftNode`, emits `workflow.definition.NodeReplaced` |
| command behaviour | `workflow.definition.ValidateDraft` | the contract is declared; the algorithm is not | given `workflow.definition.ValidateDraft` input, decide and enact exactly one outcome — `validated` otherwise, updates `workflow.definition.WorkflowDraft`, emits `workflow.definition.DraftValidated` |
| view query | `workflow.definition.DraftById` | how the projection is kept current is a storage decision | a query answering `workflow.definition.DraftById` with rows projected from `workflow.definition.WorkflowDraft` at `read_your_writes` consistency |
| view query | `workflow.definition.Drafts` | how the projection is kept current is a storage decision | a query answering `workflow.definition.Drafts` with rows projected from `workflow.definition.WorkflowDraft` at `read_your_writes` consistency |
| view query | `workflow.definition.RevisionById` | how the projection is kept current is a storage decision | a query answering `workflow.definition.RevisionById` with rows projected from `workflow.definition.WorkflowRevision` at `read_your_writes` consistency |
| view query | `workflow.definition.Revisions` | how the projection is kept current is a storage decision | a query answering `workflow.definition.Revisions` with rows projected from `workflow.definition.WorkflowRevision` at `read_your_writes` consistency |
| view query | `workflow.definition.WorkflowById` | how the projection is kept current is a storage decision | a query answering `workflow.definition.WorkflowById` with rows projected from `workflow.definition.Workflow` at `read_your_writes` consistency |
| view query | `workflow.definition.Workflows` | how the projection is kept current is a storage decision | a query answering `workflow.definition.Workflows` with rows projected from `workflow.definition.Workflow` at `read_your_writes` consistency |

## Refused — not represented by this synthesis

| capability | source | stage | why |
| --- | --- | --- | --- |
| actor grants | `workflow.definition.Author` | planning | may invoke `workflow.definition.ActivateRevision`, `workflow.definition.AddNode`, `workflow.definition.ConnectNodes`, `workflow.definition.CreateDraft`, `workflow.definition.CreateWorkflow`, `workflow.definition.DiscardDraft`, `workflow.definition.DisconnectNodes`, `workflow.definition.PublishDraft`, `workflow.definition.RemoveNode`, `workflow.definition.ReplaceNode`, `workflow.definition.ValidateDraft`; a grant is checked against a caller identity, which types do not carry, and enforcement belongs to the layer that knows who is calling |
