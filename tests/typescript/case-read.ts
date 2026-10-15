// Typed contract scope only: real provider behavior is qualified by YAI.
import type { CaseReadRequest, CaseReadResult, CaseOperation, CaseWorkStepObservation, NativeFeedbackComposition, NativeContextProjection } from "../../typescript/workflows.js";
import type { DecisionInspectInput, DecisionTrajectory, DecisionProjection } from "../../typescript/projections.js";

export const legacyDecision: DecisionProjection = { decision_id: "decision:legacy", operation_id: "operation:legacy",
  outcome: "deny", reason: "legacy basis unavailable", basis_refs: [], decision_basis: null };
export const fencedDecision: DecisionInspectInput = { case_ref: "case:exact", participant_ref: "participant:author",
  decision_ref: "decision:exact", operation_ref: "operation:exact" };
export function historicalRuleSources(trajectory: DecisionTrajectory): string[] | undefined {
  const basis = trajectory.decision.decision_basis;
  // Absence is unavailable, not a current policy substituted by the client.
  return basis?.contributing_provenance.flatMap(rule => [rule.source_id, ...rule.fact_refs, ...rule.source_locations]);
}
// @ts-expect-error A withheld Decision is not an ALLOW or a invented outcome value.
export const inventedDecision: DecisionProjection = { ...legacyDecision, outcome: "withheld_allow" };

export const query: CaseReadRequest = { read: "knowledge_search", query: "checkpoint", limit: 4 };
export const recall: CaseReadRequest = { read: "recall", query: "previous decision", limit: 4 };
export const exactRecall: CaseReadRequest = { ...recall, required_refs: ["decision:exact"] };
export const workflow: CaseReadRequest = { read: "workflow_inspect" };
export const feedback: NativeFeedbackComposition = {schema:"yai.native_feedback_composition.v1",
  mandatory_result_refs:["provider-result:source"],omitted_result_refs:[],deferred_result_refs:["provider-result:source"],
  completeness:"qualified_projections",encoding:"deferred_read_references",verbatim_feedback_bytes:18000,resident_feedback_bytes:9000,
  verbatim_request_bytes:42565,selected_request_bytes_before_pin:31565,input_unit_limit:8192};
export const expansion: NativeContextProjection = {schema:"yai.native_context_projection.v1",provider_result_id:"provider-result:inspect",
  source_provider_result_ref:"provider-result:source",source_digest:"sha256:exact",pointer:"/observation/result",value:{waiting_count:7},
  completeness:"complete_pointer_scope_not_source_or_task_sufficiency",material_is_authority:false};
export const absentExpansion: Pick<CaseWorkStepObservation,"context_projection"> = {};
export const nullExpansion: Pick<CaseWorkStepObservation,"context_projection"> = {context_projection:null};
// @ts-expect-error Caller cannot add another Case to a qualified read.
export const crossCase: CaseReadRequest = { read: "knowledge_search", query: "private", limit: 4, case_id: "case:foreign" };
// @ts-expect-error A Workflow read does not accept an unrelated query payload.
export const mismatched: CaseReadRequest = { read: "workflow_inspect", query: "ignored" };

export function sourceReferences(result: CaseReadResult): string[] {
  if (result.kind === "knowledge") return result.view.sources.map(source => source.source_id ?? source.id);
  if (result.kind === "recall") return result.trace.source_closure.map(source => source.source_ref);
  if (result.kind === "recall_insufficient") return []; // Required anchors are NOT recovered evidence.
  return [result.definition.workflow_definition_id, result.resolution.workflow_binding_id];
}
export function proposedFile(operation: CaseOperation): string | undefined {
  return operation.kind === "filesystem_write" ? operation.filesystem_write.relative_path : undefined;
}
export function nativeRequestKind(operation: CaseOperation): string | undefined {
  return typeof operation.kind === "object" ? operation.kind.resource_access : undefined;
}
export function observedStep(step: CaseWorkStepObservation): string | undefined {
  return step.read_observation?.observation_id ?? step.operation?.operation_id;
}
