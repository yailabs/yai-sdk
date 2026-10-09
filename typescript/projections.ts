import type { RecallBounds, HandoffAcceptInput, ConversationIntent, ConversationPosture, CaseWorkObservation } from "./workflows.js";
export * from "./workflows.js";
export type { CaseSummary as LiveCaseRow } from "./workflows.js";
/** Supported public Application projections. No semantic implementation.
 * Extracted from qualified consumers at Core 1820aec; Core conformance owns meaning. */
export type CognitiveCapability = "primary_conversation" | "speech_to_text" | "image_understanding";

export interface CognitivePrepareInput {
  case_ref: string; participant_ref: string; source_turn_ref: string;
  source_part_refs: string[]; capability: CognitiveCapability;
}

export interface CognitiveRealizeInput extends CognitivePrepareInput { plan_ref: string }

export interface CognitivePlan {
  plan_id: string; case_id: string; participant_id: string; case_generation: number;
  capability: CognitiveCapability; route: "native" | "derived" | "unresolved";
  role: "primary" | "auxiliary" | "none"; selected_target_id?: string;
  selected_binding_id?: string; semantic_evidence_id?: string; unresolved_reason?: string;
  arbitration?: { candidates: Array<{ target_id: string; role: string; exclusions: string[] }> };
}

export interface CognitiveComposeInput {
  case_ref: string; participant_ref: string; source_turn_ref: string;
  source_part_refs: string[]; expected_generation: number;
  prerequisite: { capability: Exclude<CognitiveCapability, "primary_conversation">; source_part_ids: string[] } | null;
}

export interface MachineAssetView {
  registration: {
    schema: "yai.machine_asset.v1";
    asset_id: string;
    integrity_digest: string;
    tenant_id: string;
    device_identity: string;
    address: string;
    port: number;
    management_user: string;
    host_public_key: string;
    approval_ref: string;
    approved_by_principal_id: string;
    approved_at_unix_ms: number;
  };
  revocation?: {
    schema: "yai.machine_revocation.v1";
    asset_id: string;
    tenant_id: string;
    device_identity: string;
    revoked_at_unix_ms: number;
    revoked_by_principal_id: string;
    reason: string;
    integrity_digest: string;
  } | null;
}

export interface MachineRegisterInput {
  tenant_id: string;
  address: string;
  port: number;
  management_user: string;
  host_public_key: string;
  approval_ref: string;
}

export interface MachineRevokeInput {
  tenant_id: string;
  asset_id: string;
  reason: string;
}

export interface RecallRequest {
  schema: "yai.recall_request.v2"; case_id: string; expected_generation: number; participant_id: string;
  at: { kind: "generation"; value: number }; query: string; required_refs: string[]; bounds: RecallBounds;
}

export interface RecalledEvidence {
  events: Array<{ event: { transition_id: string; recorded_generation: number; kind: string }; label: string; validity_at_cut: string; reasons: string[] }>;
  documentary?: { units: Array<{ unit: { id: string; source: string; text: string; kind: string; posture: string }; reasons: string[] }>; sources: Array<{ source: { id: string; source_id: string; revision_id: string; path: string }; applicable_at_cut: boolean; reasons: string[] }> } | null;
  source_closure: Array<{ source_ref: string; posture: string }>; closure_complete: boolean;
}

export interface RecallResult { trace: RecalledEvidence & { trace_id: string; request: RecallRequest; generation: number; limitations: string[]; omitted_candidates: number; expansion_stopped_at_depth: boolean } }

export interface WorkingStateRequest {
  case_id: string; expected_generation: number;
  compilation: {
    scope: { participant_id: string; purpose: "inspection"; consumer: "model"; view_kind: "model_context"; max_items: number; max_provider_claims: number; max_interaction_turns: number };
    intent: string; output_contract_id: string; max_semantic_units: number; max_derived_items: number;
    resource_refs: string[]; required_refs: string[]; previous_item_ids: string[]; view_selection_id: null;
  };
  at: { kind: "generation"; value: number } | null; recall_required_refs: string[]; recall_bounds: RecallBounds; max_output_bytes: number;
}

export interface SemanticReference { reference_id: string; family: string; members: string[]; sources: Array<{ source_id: string; revision_id: string; path: string }>; mandatory_task_dependency: boolean }

export interface WorkingEntry { entry_id: string; posture: string; value: { kind: string; value: Record<string, unknown> & { evidence?: RecalledEvidence; references?: SemanticReference[]; resident_references?: string[] } }; provenance: Array<{ kind: string; source_ref: string }> }

export interface WorkingState {
  [key: string]: unknown;
  working_state_id: string; case_id: string; case_generation: number; participant_id: string; request: WorkingStateRequest["compilation"];
  decisions?: Array<{ item_id: string; class: string; disposition: "pinned" | "retained" | "reintroduced" | "omitted"; semantic_units: number; reasons: string[] }>;
  entries: WorkingEntry[]; bounds: { selected_items: number; selected_semantic_units: number; omitted_items: number; omitted_by_locality: number; omitted_by_budget: number };
  recall?: { recall_closure_complete: boolean; limitations: string[]; omitted_recall_candidates: number };
}

export interface WorkingStateResult { working_state: WorkingState; posture?: string; compilation_mode?: string }

export interface DecisionFrontier {
  [key: string]: unknown;
  frontier_id: string; request: { max_candidates: number }; candidates: Array<{ candidate: { candidate_id: string; candidate_kind: string; description: string; semantic_refs: string[] }; requirement: string; origins: unknown[] }>;
  visible_candidate_count: number; required_candidate_count: number; omitted_optional_candidates: number;
}

export interface FrontierResult { frontier: DecisionFrontier; origin_count: number }

export interface DecisionPreparation { request_id: string; decision_kind: string; case_generation: number; candidates: DecisionFrontier["candidates"][number]["candidate"][] }

export interface DecisionPrepareInput { working_state: WorkingState; frontier: DecisionFrontier; decision_kind: string; budget: { max_candidates: number; max_result_bytes: number; max_compute_millis: number } }

export interface WorkingRefreshRequest { schema: "yai.working_refresh_request.v1"; case_id: string; participant_id: string; base_working_state_id: string; budget: null }

export interface PageRequest { schema: "yai.semantic_page_request.v1"; case_id: string; participant_id: string; base_working_state_id: string; references: string[]; action: "page_in" | "page_out"; bounds: RecallBounds }

export interface FastSearchPreparation {
  schema: "yai.fast_search_prepare_result.v1";
  availability: "unavailable_producer" | "unavailable_no_choices";
  active: boolean; actual_path: string;
  navigation: {
    navigation_id: string; working_state_id: string;
    posture: "ready_for_optional_producer" | "deterministic_fallback_no_choice";
    omitted_optional_choices: number;
    choices: Array<{
      candidate: DecisionFrontier["candidates"][number]["candidate"];
      origin: { kind: "resident_recall_group"; entry_id: string }
        | { kind: "deferred_working_group"; reference_id: string; group_entry_id: string }
        | { kind: "deterministic_path" };
    }>;
  };
}

export interface HumanCheckpoint { node_id: string; kind: "human_input"; actor_slot: string; prompt: string; required_roles: string[]; input_kind: "text"; max_bytes: number }

export interface WorkflowEdgeInput { from: string; to: string; kind: "always" }

export interface WorkflowDefinitionInput { schema: "yai.workflow_definition.v1"; tenant_id: string; workflow_key: string; declared_version: string; name: string; description: string; nodes: HumanCheckpoint[]; edges: WorkflowEdgeInput[] }

export interface WorkflowDefinition extends WorkflowDefinitionInput { workflow_definition_id: string; content_digest: string }

export interface WorkflowBindInput { case_ref: string; definition_ref: string; executor_bindings: Array<{ slot: string; participant_id: string }>; resource_bindings: Array<{ slot: string; attachment_id: string }>; case_bindings: Array<{ slot: string; case_id: string }> }

export interface WorkflowPatchInput { schema: "yai.workflow_plan_patch.v1"; base_effective_topology_digest: string; operations: Array<{ operation: "add_node"; node: HumanCheckpoint } | { operation: "add_edge"; edge: WorkflowEdgeInput } | { operation: "disable_node"; node_id: string }> }

export interface HandoffData { kind: "text"; value: string }

export interface HandoffOfferInput { source_case_ref: string; target_case_ref: string; request: HandoffData; required_target_roles: string[] }

export interface HandoffDeclineInput extends HandoffAcceptInput { reason: string }

export interface HandoffResultInput { target_case_ref: string; handoff_ref: string; participant_ref: string; outcome: "succeeded" | "failed" | "cancelled"; result: HandoffData; evidence_refs: string[] }

export interface DecisionHistoryInput { case_ref: string; participant_ref: string; max_decisions: number }

export interface DecisionInspectInput { case_ref: string; participant_ref: string; decision_ref: string }

export interface DecisionTrajectory {
  schema: "yai.cognitive_decision_trajectory.v1"; trajectory_id: string; case_id: string; participant_id: string;
  decision_transition_id: string; decision_generation: number;
  pre_decision: { cut_generation: number; content_backing: Array<{ source_ref: string; posture: string }>; unsupported_families: string[] };
  task_context?: string | null;
  candidate_posture: "exact_reconstructed" | "partial" | "unavailable";
  decision: { decision_id: string; operation_id: string; outcome: "allow" | "deny" | "require_review"; reason: string; basis_refs: string[] };
  related_evidence: Array<{ transition_id: string; recorded_generation: number }>;
  correction_decisions: string[]; missingness: string[];
  readiness: { pre_state_available: boolean; working_state_available: boolean; candidate_set_available: boolean; decision_basis_available: boolean; consequence_available: boolean; correction_available: boolean; historical_distribution_available: boolean };
}

export interface DecisionCorpus { schema: "yai.cognitive_decision_corpus.v1"; case_id: string; participant_id: string; trajectories: DecisionTrajectory[]; omitted_visible_decisions: number }

export interface DecisionEvaluation { schema: string; trajectory_count: number; exact_candidate_set_count: number; partial_candidate_set_count: number; historical_working_state_count: number; consequence_link_count: number; correction_count: number; missing_backing_count: number; temporal_leakage_violations: number; false_causality_violations: number; cross_case_leakage_violations: number; serialized_bytes: number }

export interface HandoffOffer {
  handoff_id: string; source_case_id: string; target_case_id: string;
  request: HandoffData; required_target_roles: string[]; offered_at_unix_ms: number;
}

export interface HandoffPending { schema: string; case_ref: string; generation: number; offers: HandoffOffer[]; scope: string }

export interface HandoffInspection {
  schema: string; case_ref: string; generation: number; offer: HandoffOffer;
  acceptance?: { handoff_id: string; accepted_by_participant_id: string } | null;
  decline?: { handoff_id: string; reason: string } | null;
  result?: { handoff_id: string; outcome: string; result: HandoffData; evidence_refs: string[] } | null;
  reconciliation?: { handoff_id: string; outcome: string; result?: HandoffData | null } | null;
}

export interface PolicyRule {
  kind: "operation_restriction" | "review_requirement" | "evidence_obligation" | "authority_requirement";
  rule_id: string; operation_kind: string; resource_kind?: string | null; reason: string;
  effect?: "allow" | "deny"; required?: boolean; obligation?: string;
  subject?: "proposer" | "reviewer"; required_role?: string;
  provenance: { source_id: string; fact_refs: string[]; source_locations: string[] };
}

export interface PolicyArtifactView {
  artifact: {
    artifact_id: string; policy_key: string; artifact_version: string; owner_ref: string;
    tenant_id?: string; source_id: string; source_digest: string;
    validity: { mode: string; valid_from_unix_ms?: number | null; refresh_after_unix_ms?: number | null; expires_at_unix_ms?: number | null };
    policy_ir: { compiler_version: string; ir_digest: string; rules: PolicyRule[]; unresolved: unknown[]; conflicts: Array<{ code: string; selector: string; rule_refs: string[] }> };
    validation: { status: "qualified" | "blocked"; blockers: string[]; validator_version: string };
  };
  lifecycle: "candidate" | "validated" | "published" | "superseded" | "retired" | "revoked";
  runtime_consumable: boolean; superseded_by?: string | null;
  lifecycle_events: Array<{ event_id: string; action: string; reason: string; committed_at_unix_ms: number; actor_ref: string }>;
}

export interface PolicyIngestResult { source_created: boolean; artifact_created: boolean; view: PolicyArtifactView }

export interface PolicyLifecycleResult { changed: boolean; view: PolicyArtifactView }

export type PolicyLifecycleAction = "validate" | "publish" | "retire" | "revoke";

export interface PolicyLifecycleInput { artifact_ref: string; reason: string }

export interface ConversationSendInput {
  case_ref: string; participant_ref: string; thread_ref: string;
  submission_ref: string; expected_generation: number;
  parts: Array<{ modality: "text"; media_type: "text/plain"; bytes: number[] }>;
  memory_search_mode?: "standard" | "fast";
  // Core 3802e96: ordinary SEND and bounded Case Work share one immutable intent.
  intent?: ConversationIntent;
}

export interface ConversationExecution {
  case_ref: string; participant_ref: string; submission_ref: string;
  turn_ref: string; request_ref: string; observed_generation: number;
  posture: ConversationPosture;
  invocation_refs: string[];
  primary_result?: { result_id: string; invocation_id: string; output: string; selection: { selected_target_id: string } } | null;
  attempt_outcomes: ProviderAttemptObservation[];
  work?: CaseWorkObservation | null;
}

export interface ProviderAttemptObservation {
  outcome_id?: string; target_id?: string; attempt_number?: number;
  delivery?: string; stage?: string; request_bytes_written?: number;
  response_status?: number | null; no_execution_proven?: boolean;
  failure_class?: string | null; recorded_at_unix_ms?: number;
  [key: string]: unknown;
}

export interface ConversationSubmission {
  created: boolean; execution: ConversationExecution;
  memory_search?: {
    requested: "standard" | "fast"; effective: "standard" | "fast";
    system_model_active: boolean; posture: string;
  } | null;
}

export interface PreparedFrame {
  frame_id: string; task: string; entries: WorkingEntry[];
  semantic_instructions: string[]; model_independent_constraints: string[];
}

export interface InputObservation {
  invocation_id: string; case_id: string; participant_id: string; case_generation: number;
  rendered_input_id: string; target_id: string; model_id: string;
  serialized_request_digest: string; serialized_request_bytes: number; observed_at_unix_ms: number;
  refusal: string | null;
  capacity: null | {
    public_contract: string; observation_kind: string; model_id: string;
    input_tokens: number | null; input_capacity_tokens: number; sequence_capacity_tokens: number;
    requested_output_tokens: number | null; effective_output_tokens: number | null;
    token_capacity_compatible: boolean | null; full_requested_output_fits: boolean | null;
    http_body_limit_bytes: number; resource_reservation: boolean;
  };
}

export interface PreparedContext {
  observed_generation: number; total_invocations: number; omitted_invocations: number;
  invocations: Array<{
    invocation_ref: string; lineage: { case_generation: number; rendered_input_id: string };
    working_state: WorkingState | null; projection: unknown; frame: PreparedFrame | null;
    input_observation: InputObservation | null; unavailable_reason: string | null;
  }>;
}

export type InspectedConversationExecution = ConversationExecution & { prepared_context?: PreparedContext | null };

export type ExecutionReference = { domain: "runtime_work" | "resource_request"; submission_ref: string }
  | { domain: "controlled_effect"; operation_ref: string }
  | { domain: "cognitive_realization"; plan_ref: string }
  | { domain: "cognitive_composition"; request_ref: string }
  | { domain: "source_acquisition"; source_ref: string; attempt: number };

export interface ExecutionListInput { case_ref: string; participant_ref: string; limit?: number }

export interface ExecutionListEntry { execution: ExecutionReference; recorded_at_unix_ms: number }

export interface ExecutionList { schema: string; case_ref: string; participant_ref: string; generation: number; entries: ExecutionListEntry[]; limit: number; scope: string }

export interface ExecutionGetInput { case_ref: string; participant_ref: string; execution: ExecutionReference; include_output?: boolean }

export interface EffectProposeInput { case_ref: string; participant_ref: string; resource_ref: string; candidate_ref: string; expected_generation: number }

export interface EffectSubmitInput { case_ref: string; participant_ref: string; operation_ref: string; expected_generation: number }

export interface EffectReconcileInput extends EffectSubmitInput { effect_ref: string; retry_no_effect: boolean }

export interface ControlledOperation {
  schema: string; operation_id: string; operation_digest: string; case_id: string; participant_id: string;
  kind: "filesystem_write" | "process_signal"; resource_attachment_id: string; expected_case_generation: number;
  origin: { kind: string; provider_result_id?: string };
  filesystem_write: { relative_path: string; content: string; content_digest: string; content_bytes: number };
  process_signal?: { action: string; target_identity_digest: string } | null;
}

export type EffectProposal = { posture: "recorded"; operation: ControlledOperation } | { posture: "normalization_refused"; failure: { code: string; detail: string } };

export interface SourceAcquireInput { case_ref: string; participant_ref: string; source_ref: string; attempt: number; expected_generation: number }

export interface SourceResumeInput extends SourceAcquireInput { previous_progress_ref: string }

export interface ExecutionObservation {
  case_ref: string; participant_ref: string; schema?: string; execution_ref?: string; submission_ref?: string;
  generation?: number; continuation?: ResourceRequestInput;
  process?: {
    observation_ref: string; observed_at_unix_ms: number;
    status: { exit_code: number | null; signal: number | null; timed_out: boolean; output_limit_exceeded: boolean; elapsed_ms: number; timeout_ms: number };
    output?: { stdout: string; stderr: string; stdout_digest: string; stderr_digest: string; lossy_utf8: boolean };
  };
  operation_ref?: string; source_ref?: string; attempt?: number; progress_ref?: string;
  state?: string; phase?: string; current_source_phase?: string; observed_generation?: number;
  posture?: string | { state: string; result_ref?: string; receipt_ref?: string; effect_ref?: string; outcome?: string; review_ref?: string; decision_ref?: string; external_execution_started?: boolean };
  runner?: { run_ref: string; checkpoint_digest: string; posture: string; stop_requested: boolean };
  progress?: { status: "normalization_rejected" | "denied" | "awaiting_review" | "finalized" | "indeterminate"; operation_id?: string | null; decision_id?: string | null; review_id?: string | null; effect_id?: string | null; receipt_id?: string | null; outcome?: string | null } | null;
  plan_ref?: string; request_ref?: string;
  provider_result?: { result_id: string; output: string; invocation_id: string } | null;
  attempt_outcomes?: ProviderAttemptObservation[];
}

export interface ExecutionSubmission { created?: boolean; execution: ExecutionObservation; advancement?: string; outcome?: { posture: string; reason?: string } }

export interface RuntimeBudgets { max_invocations: number; max_operations: number; max_semantic_units: number; max_resident_items: number; max_estimated_input_units: number; max_provider_retries: number; max_runtime_ms: number; stop_on_deny: boolean; continue_after_malformed: boolean }

export interface CaseRunInput { case_ref: string; participant_ref: string; resource_ref: string; submission_ref: string; task: string; budgets: RuntimeBudgets }

export interface CaseStopInput { case_ref: string; participant_ref: string; submission_ref: string; run_ref: string }

export type ResourceAction = { action: "filesystem_read" | "discover"; path: string }
  | { action: "filesystem_search"; path: string; needle: string }
  | { action: "process_run" | "database_query" | "database_mutation" | "http_fetch"; name: string }
  | { action: "mcp_catalog" };

export interface ResourceRequestInput { case_ref: string; participant_ref: string; resource_ref: string; submission_ref: string; expected_generation: number; request: { schema: "yai.resource_request.v1"; configuration_digest: string; action: ResourceAction } }

export interface CaseResumeInput { case_ref: string; participant_ref: string; previous_submission_ref: string; submission_ref: string; run_ref: string; checkpoint_digest: string; budgets: RuntimeBudgets }

export interface ProviderRegistration {
  tenant_id: string; provider_key: string; adapter: "open_ai_compatible";
  endpoint: string; model_id: string; credential_ref: string;
  locality: "loopback" | "private_network" | "remote"; extension_adapter_id?: "yvex.http.v1";
}

export interface ProviderTarget extends ProviderRegistration { target_id: string; integrity_digest: string }

export interface ProviderProbeEvidence {
  run_id: string; target_id: string; started_at_unix_ms: number; completed_at_unix_ms: number;
  transport_connected: boolean; exact_model_addressed: boolean; chat_text_envelope_valid: boolean;
  structured_json_object_valid: boolean; usage_accounting_observed: boolean;
  health_endpoint_observed: boolean; extension_telemetry_observed: boolean;
  text_embedding_envelope_valid?: boolean; embedding_dimension?: number;
  realization_shapes?: string[]; failure_codes: string[];
}

export interface ProviderQualificationInput { target_ref: string; evidence: ProviderProbeEvidence; suite_ref: string; valid_until_unix_ms?: number }

export interface ProviderQualification { qualification_id: string; target_id: string; suite_id: string; run_id: string; evidence: ProviderProbeEvidence; capabilities: Array<{ capability: string; provenance: string; evidence_refs: string[] }> }

export interface ProviderBindingInput { case_ref: string; participant_ref: string; ordered_target_refs: string[]; failover_policy: "none" | "safe_only"; max_attempts_per_turn: number }

export interface ProviderPosture {
  qualification?: { id: string; suite_id: string; run_id: string; qualified_at_unix_ms: number; valid_until_unix_ms?: number; capabilities: Array<{ capability: string; provenance: string; evidence_refs: string[] }> };
  trust?: { event_ref: string; posture: string; recorded_at_unix_ms: number };
  health: { posture: string; effective_posture?: string; evaluated_at_unix_ms?: number | null;
    circuit: string; consecutive_failures: number; observed_at_unix_ms?: number; failure_class?: string };
}

export interface SemanticEvidence { evidence_id: string; target_id: string; capability: string; posture: string; suite_id: string; run_id: string }

export interface CognitiveBinding { binding_id: string; participant_id: string; role: string; capability: string; target_id: string; semantic_evidence_id: string;
  target_policy?: { kind: "ordered_eligible"; alternatives: Array<{ target_id: string; target_digest: string; semantic_evidence_id: string }> } }

export interface SuitabilityInput { target_ref: string; capability: "primary_conversation"; suite_ref: string; run_ref: string; evidence_refs: string[] }

export interface CognitiveBindingInput { case_ref: string; participant_ref: string; role: "primary"; capability: "primary_conversation"; candidates: Array<{ target_ref: string; semantic_evidence_ref: string }>; replace: boolean }

export interface ProviderConnectionModelsInput { tenant_id: string; endpoint: string; locality: ProviderRegistration["locality"]; credential_ref: string }

export type ProviderModelsInput = ProviderConnectionModelsInput | { tenant_id: string; target_ref: string };

export interface ProviderModels { target_ref?: string | null; observed_at_unix_ms?: number; models: string[]; capacity?: ProviderCatalogCapacity | null; scope: "currently_exposed"; authority: "provider_metadata_only" }

export interface ProviderCatalogCapacity {
  public_contract: "yvex.openai.compat.v3"; observation_kind: "public_model_catalog";
  model_id: string; engine_generation: number;
  runtime_binding_identity: string; runtime_model_identity: string; capacity_plan_identity: string;
  input_capacity_tokens: number; sequence_capacity_tokens: number; http_body_limit_bytes: number;
  resource_reservation: false; execution_or_resources_qualified: false;
}

export interface ProviderProbeInput {
  target_ref: string; submission_ref: string; embedding: boolean;
  realization_shapes: Array<"text_to_text" | "text_functions_to_text_or_call" | "text_to_json_object">;
  qualify: boolean; valid_for_ms?: number;
}

export interface ProviderProbeExecution {
  schema: "yai.provider_probe_execution.v1"; target_ref: string; submission_ref: string;
  created: boolean; posture: "running" | "completed" | "failed" | "interrupted";
  run: {
    request: { target_id: string; submission_ref: string; embedding: boolean; realization_shapes: string[]; qualify: boolean; valid_for_ms?: number | null };
    evidence?: ProviderProbeEvidence | null; qualification?: ProviderQualification | null;
    failure_code?: string | null;
    owner: { started_at_unix_ms: number };
  };
}

export interface ProviderProbeList { schema: "yai.provider_probe_list.v1"; target_ref: string; runs: ProviderProbeExecution[] }

export interface TenantPresentation { membership: string; tenant: { tenant_id: string; organization_ref: string } }

export interface SourceDeclarationInput {
  case_ref: string; perimeter: string; logical_name: string; participant_ref: string; resource_ref: string;
  roles: Array<"knowledge" | "operational" | "policy">;
  action: { action: "discover"; path: string } | { action: "database_query" | "http_fetch"; name: string };
  bootstrap_policy: boolean; media_type: string;
}

export interface ResourceImportInput {
  case_ref: string;
  definition: {
    schema: "yai.resource_definition.v1"; attachment_id: string; policy_owner: string;
    participant_ids: string[]; operations: string[]; read_prefixes: string[]; names: string[];
    max_output_bytes: number; max_items: number; review_requirement: "require_review";
    address: { kind: "filesystem" | "discovery"; root: string }
      | { kind: "process_runner"; root: string; runners: Record<string, { executable: string; executable_digest: string; argv: string[]; working_directory: string; environment: Record<string, string>; timeout_ms: number }> }
      | { kind: "sqlite"; root: string; path: string; queries: Record<string, string> }
      | { kind: "http_service"; endpoint: NetworkResourceInput; paths: Record<string, string> }
      | { kind: "mcp"; endpoint: NetworkResourceInput };
  };
}

export interface NetworkResourceInput { endpoint: string; allowed_ip_addresses: string[]; credential_ref: string | null }

export interface CasePolicyBindingInput { case_ref: string; artifact_ref: string; expected_generation: number; reason: string }

export interface CasePolicyReplacementInput extends CasePolicyBindingInput { prior_binding_ref: string }

export interface CasePolicyUnbindingInput { case_ref: string; binding_ref: string; expected_generation: number; reason: string }

export type ResourceOperationKind = "filesystem_write" | "process_signal" | { resource_access: string };

export interface CaseCapabilityView {
  case_id: string; case_generation: number; participant_id: string; view_id: string; effective_policy_id: string;
  entries: Array<{ resource: { attachment_id: string }; operation_kind: ResourceOperationKind; requires_current_decision: boolean; policy_constraints: Array<{ kind: string; effect?: string; required?: boolean; resolution: string }> }>;
  exclusions: Array<{ resource_id: string; operation_kind: ResourceOperationKind; reason: string }>;
}

export interface LiveNode { id: string; label?: string; kind?: string; detail?: string }

export interface LiveEdge { id: string; from: string; to: string; kind: string; from_kind?: string; to_kind?: string }

export interface TimelineEntry {
  id: string; sequence: number; committed_at_unix_ms: number; kind: string;
  participant_ref?: string; component: string; causal_refs: string[]; summary?: string;
}

export interface LiveWorkspace {
  case: {
    case_ref: string; display_name: string; case_status: string; generation: number;
    tenant_ref?: string; participant_ref: string; updated_at_unix_ms?: number;
  };
  overview: {
    attention: Array<{ kind: string; title: string; detail: string; ref?: string }>;
    participants: Array<{ id: string; roles: string[]; is_current: boolean; model_context_admitted?: boolean | null }>;
  };
  environment: {
    sources: Array<{ id: string; label: string; kind: string; perimeter: string; media_type: string; roles: string[]; resource_ref: string; posture?: string; revision_ref?: string; items?: number; attempt?: number; progress_ref?: string }>;
    files: Array<{ id: string; source_ref: string; source_label: string; revision_ref: string; path: string; digest: string; bytes: number; media_type: string; backing: unknown }>;
    resources: Array<{ id: string; label?: string; kind: string; policy_ref: string; review_requirement: string; allowed_write_prefix: string; max_write_bytes: number; operations: string[]; read_prefixes: string[]; names: string[]; max_output_bytes?: number; max_items?: number; configuration_digest?: string }>;
    artifacts: unknown[];
  };
  knowledge: {
    status: string; message: string; profile?: string; source_closure?: string;
    sources: Array<{ id: string; source_ref: string; label: string; revision_ref: string; path: string; digest: string; media_type: string; extractor: string; status: string; detail: string }>;
    units: Array<{ id: string; source_ref: string; parent_ref?: string; kind: string; text: string; posture: string; entity_ref?: string; predicate?: string; value?: unknown; references: string[]; topics: string[] }>;
    entities: Array<{ id: string; definitions: string[] }>;
    topics: Array<{ name: string; units: string[] }>;
    contradictions: Array<{ id: string; entity: string; predicate: string; members: string[]; posture: string }>;
    relations: Array<LiveEdge & { posture?: string; backing_units?: string[] }>;
  };
  memory: { authority: string; timeline: TimelineEntry[]; relations: LiveEdge[]; generation: number };
  authority: { policies: Array<{ id: string; policy_key: string; lineage_ref: string; artifact_ref: string; source_ref: string; owner_ref: string; version: string; bound_at_generation: number; reason: string }>; reviews: Array<{ id: string; status: string; operation_ref: string; policy_ref: string; decision_ref?: string; evidence_ref?: string; required_roles: string[] }>; grants: Record<string, unknown>[]; last_decision?: Record<string, unknown>; empty: boolean };
  work: {
    status: string;
    message?: string;
    definition?: { name?: string; description?: string; nodes?: unknown[] };
    effective_nodes?: Array<{ node_id: string; instance_path: string; workflow_definition_id: string; local_node_id: string; node: { kind: string; prompt?: string; max_bytes?: number } }>;
    resolution?: { effective_topology_digest?: string; completed?: boolean; nodes: Array<{ node_id: string; node_kind: string; posture: string; reason: string; evidence_refs: string[] }> };
    nodes: Array<{ node_id: string; node_kind: string; posture: string; reason: string }>;
    edges: LiveEdge[];
  };
  compute: { cognitive_bindings?: CognitiveBinding[]; status: string; message: string; targets: Array<{ id: string; provider_key: string; adapter: string; model_id: string; locality: string; endpoint: string; posture?: ProviderPosture | string; management: string; extension_adapter_id?: string | null; semantic_evidence?: SemanticEvidence[] }> };
  conversation: { read_only: boolean; turns: Array<{ id: string; thread_ref: string; participant_ref: string; generation: number; execution_request_ref?: string | null; parts: Array<{ part_ref?: string; modality: string; media_type: string; text?: string }> }> };
  freshness: { generation: number; resync_operation: string };
}

export interface MaterialReadProjection {
  case_ref: string;
  generation: number;
  source_ref: string;
  revision_ref: string;
  path: string;
  digest: string;
  bytes: number;
  media_type: string;
  encoding: "utf-8" | "base64";
  content: string;
}
