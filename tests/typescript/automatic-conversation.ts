import type { ProgressiveConversationInput, AutomaticConversationObservation } from "../../typescript/workflows.js";
export const ordinary: ProgressiveConversationInput = {
  execution: "automatic", case_ref: "case:existing", participant_ref: "participant:author",
  thread_ref: "thread:existing", submission_ref: "send:fresh", expected_generation: 1,
  parts: [{ modality: "text", media_type: "text/plain", bytes: [65] }], progressive_output: true, memory_search_mode: "standard"
};
export function consequence(value: AutomaticConversationObservation): string {
  return `${value.binding_ref}:${value.consequence}:${value.limits?.effects ?? "no_work_profile"}`;
}
export const textCompatible: ProgressiveConversationInput = {...ordinary, execution:"automatic_v2"};
