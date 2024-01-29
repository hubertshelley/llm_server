use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", tag = "role")]
pub enum ChatCompletionMessage {
    /// A message from a system.
    System(SystemMessage),
    /// A message from a human.
    User(UserMessage),
    /// A message from the assistant.
    Assistant(AssistantMessage),
    /// A message from a tool.
    Tool(ToolMessage),
}
#[derive(Debug, Clone, Deserialize)]
pub struct SystemMessage {
    /// The contents of the system message.
    pub(crate) content: String,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserMessage {
    /// The contents of the user message.
    pub(crate) content: String,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssistantMessage {
    /// The contents of the system message.
    #[serde(default)]
    pub content: Option<String>,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// The tool calls generated by the model, such as function calls.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tool_calls: Vec<ToolCall>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolMessage {
    /// The contents of the tool message.
    content: String,
    /// Tool call that this message is responding to.
    tool_call_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolCall {
    /// The ID of the tool call.
    pub id: String,
    /// The type of the tool. Currently, only function is supported.
    pub r#type: ToolType,
    /// The function that the model called.
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionCall {
    /// The name of the function to call.
    pub name: String,
    /// The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function.
    pub arguments: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    #[default]
    Function,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ChatResponseFormatObject {
    pub(crate) r#type: ChatResponseFormat,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatResponseFormat {
    Text,
    #[default]
    Json,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Tool {
    /// The schema of the tool. Currently, only functions are supported.
    r#type: ToolType,
    /// The schema of the tool. Currently, only functions are supported.
    function: FunctionInfo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FunctionInfo {
    /// A description of what the function does, used by the model to choose when and how to call the function.
    description: String,
    /// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    name: String,
    /// The parameters the functions accepts, described as a JSON Schema object.
    parameters: serde_json::Value,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoice {
    #[default]
    None,
    Auto,
    // TODO: we need something like this: #[serde(tag = "type", content = "function")]
    Function {
        name: String,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatCompletionChoice {
    /// The reason the model stopped generating tokens. This will be stopped if the model hit a natural stop point or a provided stop sequence, length if the maximum number of tokens specified in the request was reached, content_filter if content was omitted due to a flag from our content filters, tool_calls if the model called a tool, or function_call (deprecated) if the model called a function.
    pub finish_reason: FinishReason,
    /// The index of the choice in the list of choices.
    pub index: usize,
    /// A chat completion message generated by the model.
    pub message: AssistantMessage,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatCompleteUsage {
    /// Number of tokens in the generated completion.
    pub completion_tokens: usize,
    /// Number of tokens in the prompt.
    pub prompt_tokens: usize,
    /// Total number of tokens used in the request (prompt + completion).
    pub total_tokens: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    #[default]
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
    Null,
}
