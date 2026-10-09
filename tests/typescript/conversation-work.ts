// Public representation only: no Core admission, provider or Studio proof.
import type {
  CaseWorkLimits, CaseWorkObservation, ConversationExecution, ConversationIntent,
  ConversationPosture, ConversationSendInput, ConversationSubmission,
  InspectedConversationExecution, ConversationWorkResumeInput,
  ConversationObservation, ConversationProgressObservation,
} from "../../typescript/projections.js";

type Equal<A, B> = (<T>() => T extends A ? 1 : 2) extends
  (<T>() => T extends B ? 1 : 2) ? true : false;
type Assert<T extends true> = T;

export type IntentHasOneOwner = Assert<Equal<ConversationSendInput["intent"], ConversationIntent | undefined>>;
export type PostureHasOneOwner = Assert<Equal<ConversationExecution["posture"], ConversationPosture>>;
export type WorkHasOneOwner = Assert<Equal<ConversationExecution["work"], CaseWorkObservation | null | undefined>>;
export type ProgressOptInPreserved = Assert<Equal<ConversationSendInput["progressive_output"], boolean | undefined>>;

// Ordinary SEND stays ordinary. Focused context is not a Work preset.
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

// Explicit intent shares the generated owner, including author/executor and
// Workflow binding. Core, not this type fixture, decides compatibility.
export const work: ConversationSendInput = {
  ...ordinary, submission_ref: "send:work", memory_search_mode: "standard",
  intent: {
    executor_participant_id: "participant:model",
    work_limits: { ...limits, max_output_tokens: 96 },
    workflow_execution_id: "workflow-execution:admitted",
  },
};
export const progressiveWork: ConversationSendInput = { ...work, progressive_output: true };

export const review: ConversationSubmission = {
  created: true,
  execution: {
    case_ref: "case:isolated", participant_ref: "participant:author",
    submission_ref: "send:work", turn_ref: "turn:exact", request_ref: "request:exact",
    observed_generation: 9, posture: "awaiting_review", invocation_refs: ["invocation:exact"],
    primary_result: null, attempt_outcomes: [],
    work: {
      schema: "yai.case_work_execution.v1", request_ref: "request:exact",
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

// Exact Core absence is null; older omitted fields remain compatible.
export const noAnswer: CaseWorkObservation = {
  ...review.execution.work!, answer: null,
  steps: [{ ordinal: 1, source_ref: "step:pending", selection_ref: "selection:pending",
    target_ref: "target:exact", invocation_ref: null, provider_result_ref: null,
    operation_ref: null, outcome_refs: [] }],
};

export const admittedWithoutResult: ConversationObservation = {
  case_ref: "case:isolated", participant_ref: "participant:author",
  submission_ref: "send:pending", turn_ref: "turn:pending", request_ref: "request:pending",
  observed_generation: 10, posture: "admitted", invocation_refs: [],
  primary_result: null, attempt_outcomes: [],
};

export const uncertain: InspectedConversationExecution = {
  ...review.execution, posture: "delivery_indeterminate", primary_result: null,
  work: { ...review.execution.work!, posture: "delivery_indeterminate" },
  prepared_context: null,
};

// Provisional fragments are not a Work answer or a committed model result.
export type CanonicalProgressExecution = Assert<Equal<ConversationProgressObservation["execution"], ConversationObservation>>;
export const partial: ConversationProgressObservation = {
  schema: "yai.conversation_progress.v1", case_ref: "case:isolated",
  participant_ref: "participant:author", executor_participant_ref: "participant:model",
  thread_ref: "thread:isolated", turn_ref: "turn:pending", request_ref: "request:pending",
  availability: "live", stream_ref: "stream:exact",
  oldest_sequence: 1, last_sequence: 1, history_gap: false,
  events: [], partial_output: "provisional", partial_output_limited: false,
  execution: admittedWithoutResult,
  current_invocation_ref: null, current_target_ref: null,
};

// Continuation retains the original submission and an exact generation fence.
export const resume: ConversationWorkResumeInput = {
  case_ref: "case:isolated", participant_ref: "participant:author",
  submission_ref: "send:work", observed_generation: 9,
};

// @ts-expect-error mandatory bounds are not invented by the public client.
export const incompleteLimits: CaseWorkLimits = { invocations: 2, operations: 1, max_input_units: 8192 };
export const escalated: ConversationIntent = {
  // @ts-expect-error assignment or intent is not a permission or Grant.
  bypass_authority: true,
};
// @ts-expect-error model assertions do not create canonical completion.
export const fabricatedSuccess: ConversationPosture = "model_says_done";
