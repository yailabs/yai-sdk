// Public client conformance, not a Core execution or real-provider proof.
import type {
  CaseWorkLimits, CaseWorkObservation, ConversationExecution, ConversationIntent,
  ConversationPosture, ConversationSendInput, ConversationSubmission,
  InspectedConversationExecution, ConversationWorkResumeInput,
} from "../../typescript/projections.js";

type Equal<A, B> = (<T>() => T extends A ? 1 : 2) extends
  (<T>() => T extends B ? 1 : 2) ? true : false;
type Assert<T extends true> = T;

export type IntentHasOneOwner = Assert<Equal<ConversationSendInput["intent"], ConversationIntent | undefined>>;
export type PostureHasOneOwner = Assert<Equal<ConversationExecution["posture"], ConversationPosture>>;
export type WorkHasOneOwner = Assert<Equal<ConversationExecution["work"], CaseWorkObservation | null | undefined>>;

// Old ordinary SEND remains valid, with no implicit work or executor identity.
export const ordinary: ConversationSendInput = {
  case_ref: "case:isolated", participant_ref: "participant:author",
  thread_ref: "thread:isolated", submission_ref: "send:ordinary",
  expected_generation: 3,
  parts: [{ modality: "text", media_type: "text/plain", bytes: [72, 105] }],
  intent: { context_depth: "focused" },
};

export const limits: CaseWorkLimits = {
  invocations: 2, operations: 1, effects: 0, max_input_units: 8192,
};

// Work is explicit, bounded, and may distinguish author from executor.
export const work: ConversationSendInput = {
  ...ordinary, submission_ref: "send:work", memory_search_mode: "fast",
  intent: {
    context_depth: "focused", executor_participant_id: "participant:model",
    work_limits: { ...limits, max_output_tokens: 96 },
    workflow_execution_id: "workflow-execution:admitted",
  },
};

export const review: ConversationSubmission = {
  created: true,
  execution: {
    case_ref: "case:isolated", participant_ref: "participant:author",
    submission_ref: "send:work", turn_ref: "turn:exact", request_ref: "request:exact",
    observed_generation: 9, posture: "awaiting_review", invocation_refs: ["invocation:exact"],
    primary_result: null, attempt_outcomes: [],
    work: {
      schema: "yai.case_work_observation.v1", request_ref: "request:exact",
      participant_ref: "participant:model", thread_ref: "thread:isolated",
      observed_generation: 9, posture: "awaiting_review",
      steps: [{ ordinal: 0, source_ref: "step:exact", selection_ref: "selection:exact",
        target_ref: "target:exact", invocation_ref: "invocation:exact",
        provider_result_ref: "result:proposal", operation_ref: "operation:proposed",
        outcome_refs: [] }],
    },
  },
};

export const exhausted: ConversationExecution = {
  ...review.execution, posture: "budget_exhausted",
  work: { ...review.execution.work!, posture: "budget_exhausted" },
};

export const uncertain: InspectedConversationExecution = {
  ...review.execution, posture: "delivery_indeterminate", primary_result: null,
  work: { ...review.execution.work!, posture: "delivery_indeterminate" },
  prepared_context: null,
};

// Exact continuation is not a second SEND. Core still decides resumability.
export const resume: ConversationWorkResumeInput = {
  case_ref: "case:isolated", participant_ref: "participant:author",
  submission_ref: "send:work", observed_generation: 9,
};

// Typed clients may not drop a required bound or add client-owned authority.
// @ts-expect-error effects is a mandatory work bound, never an assumed zero.
export const incompleteLimits: CaseWorkLimits = { invocations: 2, operations: 1, max_input_units: 8192 };
export const escalated: ConversationIntent = {
  // @ts-expect-error an intent does not contain a permission or Grant.
  bypass_authority: true,
};
// @ts-expect-error a model assertion is not a supported execution posture.
export const fabricatedSuccess: ConversationPosture = "model_says_done";
