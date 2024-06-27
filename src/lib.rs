use extism_pdk::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, str::from_utf8};

#[derive(Serialize)]
struct ToolFunction {
    name: String,
    description: String,
    parameters: FunctionParameters,
}

#[derive(Serialize)]
struct FunctionParameters {
    #[serde(rename = "type")]
    param_type: String,
    properties: HashMap<String, serde_json::Value>,
    required: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolResult {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    function: ToolFunctionResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolFunctionResult {
    name: String,
    arguments: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatResult {
    result: ChatResponse,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatResponse {
    response: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ToolCall {
    arguments: Option<HashMap<String, serde_json::Value>>,
    name: String,
}

#[derive(Debug)]
struct CloudflareAIConfig {
    account_id: String,
    api_key: String,
    model: Model,
    temperature: f32,
    role: String,
}

#[derive(Clone, Debug, Serialize)]
struct Model {
    name: &'static str,
    aliases: [&'static str; 1],
}

#[derive(Serialize, Deserialize, FromBytes)]
#[encoding(Json)]
pub struct CompletionToolInput {
    pub tools: Vec<Tool>,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, FromBytes)]
#[encoding(Json)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, FromBytes)]
#[encoding(Json)]
pub struct Tool {
    pub name: Option<String>,
    pub description: Option<String>,
    pub input_schema: InputSchema,
    #[serde(default = "default_type")]
    pub r#type: String,
}

fn default_type() -> String {
    "function".to_string()
}

#[derive(Serialize, Deserialize, FromBytes)]
#[encoding(Json)]
pub struct InputSchema {
    #[serde(rename = "type")]
    pub data_type: String,
    pub properties: HashMap<String, serde_json::Value>,
    pub required: Vec<String>,
}

static MODELS: [Model; 36] = [
    Model {
        name: "@cf/meta/llama-3-8b-instruct",
        aliases: ["llama-3-8b"],
    },
    Model {
        name: "@cf/meta/llama-2-7b-chat-fp16",
        aliases: ["llama-2-7b"],
    },
    Model {
        name: "@cf/meta/llama-2-7b-chat-int8",
        aliases: ["llama-2-7b-int8"],
    },
    Model {
        name: "@cf/mistral/mistral-7b-instruct-v0.1",
        aliases: ["mistral-7b"],
    },
    Model {
        name: "@hf/thebloke/deepseek-coder-6.7b-base-awq",
        aliases: ["deepseek-coder-6.7b"],
    },
    Model {
        name: "@hf/thebloke/deepseek-coder-6.7b-instruct-awq",
        aliases: ["deepseek-coder-6.7b-instruct"],
    },
    Model {
        name: "@cf/deepseek-ai/deepseek-math-7b-base",
        aliases: ["deepseek-math-7b"],
    },
    Model {
        name: "@cf/deepseek-ai/deepseek-math-7b-instruct",
        aliases: ["deepseek-math-7b-instruct"],
    },
    Model {
        name: "@cf/thebloke/discolm-german-7b-v1-awq",
        aliases: ["discolm-german-7b-v1-awq"],
    },
    Model {
        name: "@cf/tiiuae/falcon-7b-instruct",
        aliases: ["falcon-7b"],
    },
    Model {
        name: "@cf/google/gemma-2b-it-lora",
        aliases: ["gemma-2b"],
    },
    Model {
        name: "@cf/google/gemma-7b-it",
        aliases: ["gemma-7b"],
    },
    Model {
        name: "@cf/google/gemma-7b-it-lora",
        aliases: ["gemma-7b-lora"],
    },
    Model {
        name: "@hf/nousresearch/hermes-2-pro-mistral-7b",
        aliases: ["hermes-2-pro-mistral-7b"],
    },
    Model {
        name: "@hf/thebloke/llama-2-13b-chat-awq",
        aliases: ["llama-2-13b-chat-awq"],
    },
    Model {
        name: "@hf/thebloke/llama-2-13b-chat-awq",
        aliases: ["llama-2-13b-chat-awq"],
    },
    Model {
        name: "@cf/meta-llama/llama-2-7b-chat-hf-lora",
        aliases: ["llama-2-7b-chat-hf-lora"],
    },
    Model {
        name: "@cf/meta/llama-3-8b-instruct",
        aliases: ["llama-3-8b-instruct"],
    },
    Model {
        name: "@cf/meta/llama-3-8b-instruct-awq",
        aliases: ["llama-3-8b-instruct-awq"],
    },
    Model {
        name: "@hf/thebloke/llamaguard-7b-awq",
        aliases: ["llamaguard-7b-awq"],
    },
    Model {
        name: "@hf/thebloke/mistral-7b-instruct-v0.1-awq",
        aliases: ["mistral-7b-instruct-v0.1-awq"],
    },
    Model {
        name: "@hf/mistral/mistral-7b-instruct-v0.2",
        aliases: ["mistral-7b-instruct-v0.2"],
    },
    Model {
        name: "@cf/mistral/mistral-7b-instruct-v0.2-lora",
        aliases: ["mistral-7b-instruct-v0.2-lora"],
    },
    Model {
        name: "@hf/thebloke/neural-chat-7b-v3-1-awq",
        aliases: ["neural-chat-7b-v3-1-awq"],
    },
    Model {
        name: "@cf/openchat/openchat-3.5-0106",
        aliases: ["openchat-3.5-0106"],
    },
    Model {
        name: "@hf/thebloke/openhermes-2.5-mistral-7b-awq",
        aliases: ["openhermes-2.5-mistral-7b-awq"],
    },
    Model {
        name: "@cf/microsoft/phi-2",
        aliases: ["phi-2"],
    },
    Model {
        name: "@cf/qwen/qwen1.5-0.5b-chat",
        aliases: ["qwen1.5-0.5b-chat"],
    },
    Model {
        name: "@cf/qwen/qwen1.5-1.8b-chat",
        aliases: ["qwen1.5-1.8b-chat"],
    },
    Model {
        name: "@cf/qwen/qwen1.5-14b-chat-awq",
        aliases: ["qwen1.5-14b-chat-awq"],
    },
    Model {
        name: "@cf/qwen/qwen1.5-7b-chat-awq",
        aliases: ["qwen1.5-7b-chat-awq"],
    },
    Model {
        name: "@cf/defog/sqlcoder-7b-2",
        aliases: ["sqlcoder-7b-2"],
    },
    Model {
        name: "@hf/nexusflow/starling-lm-7b-beta",
        aliases: ["starling-lm-7b-beta"],
    },
    Model {
        name: "@cf/tinyllama/tinyllama-1.1b-chat-v1.0",
        aliases: ["tinyllama-1.1b-chat-v1.0"],
    },
    Model {
        name: "@cf/fblgit/una-cybertron-7b-v2-bf16",
        aliases: ["una-cybertron-7b-v2-bf16"],
    },
    Model {
        name: "@hf/thebloke/zephyr-7b-beta-awq",
        aliases: ["zephyr-7b-beta-awq"],
    },
];

fn get_completion(
    api_key: String,
    model: &Model,
    prompt: String,
    temperature: f32,
    role: String,
    account_id: String,
    tools: Option<Vec<Tool>>,
) -> Result<ChatResult, anyhow::Error> {
    let req = HttpRequest::new(format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/ai/run/{}",
        account_id, model.name
    ))
    .with_header("Authorization", format!("Bearer {}", api_key))
    .with_header("Content-Type", "application/json")
    .with_method("POST");

    let mut wrapped_tools: Vec<ToolFunction> = Vec::new();
    match tools {
        Some(tools) => {
            info!("Tools found");
            wrapped_tools = tools
                .into_iter()
                .map(|tool| ToolFunction {
                    name: tool.name.unwrap_or_default(),
                    description: tool.description.unwrap_or_default(),
                    parameters: FunctionParameters {
                        param_type: tool.input_schema.data_type,
                        properties: tool.input_schema.properties,
                        required: tool.input_schema.required,
                    },
                })
                .collect();
        }
        None => {
            info!("No tools found");
        }
    }

    let mut req_body = json!({
        "temperature": temperature,
        "messages": [
            {
                "role": "system",
                "content": role,
            },
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    if !wrapped_tools.is_empty() {
        req_body["tools"] = json!(wrapped_tools);
    }

    info!("Request body: {}", req_body.to_string());

    let res = http::request::<String>(&req, Some(req_body.to_string()))?;

    match res.status_code() {
        200 => {
            info!("Request successful");
        }
        _ => {
            let response_body = res.body();
            let body = from_utf8(&response_body)?;
            return Err(anyhow::anyhow!(
                "error calling API\nStatus Code: {}\n Response: {}",
                res.status_code(),
                body
            ));
        }
    }
    let response_body = res.body();
    let body = from_utf8(&response_body)?;

    let chat_result: ChatResult = serde_json::from_str(body)?;
    Ok(chat_result)
}
fn get_config_values(
    cfg_get: impl Fn(&str) -> Result<Option<String>, anyhow::Error>,
) -> FnResult<CloudflareAIConfig> {
    let api_key = cfg_get("api_key")?;
    let account_id_input = cfg_get("account_id")?;
    let model_input = cfg_get("model")?;
    let temperature_input = cfg_get("temperature")?;
    let role_input = cfg_get("role")?;

    match api_key {
        Some(_) => {
            info!("API key found");
        }
        None => {
            error!("API key not found");
            return Err(WithReturnCode::new(anyhow::anyhow!("API key not found"), 1));
        }
    }

    let model = match model_input {
        Some(model) => {
            let found_model = MODELS.iter().find(|m| {
                m.name.to_lowercase() == model.to_lowercase()
                    || m.aliases
                        .iter()
                        .any(|&alias| alias.to_lowercase() == model.to_lowercase())
            });
            match found_model {
                Some(m) => {
                    info!("Model found: {}", m.name);
                    m
                }
                None => {
                    error!("Model not found");
                    return Err(WithReturnCode::new(anyhow::anyhow!("Model not found"), 1));
                }
            }
        }
        _ => {
            info!("Model not specified, using default");
            MODELS.first().unwrap()
        }
    };

    let temperature = match temperature_input {
        Some(temperature) => {
            let t = temperature.parse::<f32>();
            match t {
                Ok(t) => {
                    if t < 0.0 || t > 1.0 {
                        error!("Temperature must be between 0.0 and 1.0");
                        return Err(WithReturnCode::new(
                            anyhow::anyhow!("Temperature must be between 0.0 and 1.0"),
                            1,
                        ));
                    }
                    info!("Temperature: {}", t);
                    t
                }
                Err(_) => {
                    error!("Temperature must be a float");
                    return Err(WithReturnCode::new(
                        anyhow::anyhow!("Temperature must be a float"),
                        1,
                    ));
                }
            }
        }
        None => {
            info!("Temperature not specified, using default");
            0.7
        }
    };

    let role = role_input.unwrap_or("".to_string());
    if role != "" {
        info!("Role: {}", role);
    } else {
        info!("Role not specified");
    }

    let account_id = account_id_input.unwrap_or("".to_string());
    if account_id != "" {
        info!("Account ID: {}", account_id);
    } else {
        error!("Account ID not specified");
        return Err(WithReturnCode::new(
            anyhow::anyhow!("Account ID not specified"),
            1,
        ));
    }

    Ok(CloudflareAIConfig {
        account_id: account_id,
        api_key: api_key.unwrap(),
        model: model.clone(),
        temperature,
        role,
    })
}

#[plugin_fn]
pub fn completion(input: String) -> FnResult<String> {
    let cfg = get_config_values(|key| config::get(key))?;

    let res = get_completion(
        cfg.api_key,
        &cfg.model,
        input,
        cfg.temperature,
        cfg.role,
        cfg.account_id,
        None,
    )?;

    let output = res.result.response;

    Ok(output.unwrap_or_default())
}

#[plugin_fn]
pub fn completionWithTools(input: CompletionToolInput) -> FnResult<String> {
    let cfg = get_config_values(|key| config::get(key))?;

    let prompt = input.messages[0].content.clone();
    let res = get_completion(
        cfg.api_key,
        &cfg.model,
        prompt,
        cfg.temperature,
        cfg.role,
        cfg.account_id,
        Some(input.tools),
    )?;

    info!("Response: {:?}", res);

    let tool_calls = res
        .result
        .tool_calls
        .as_ref()
        .ok_or(anyhow::anyhow!("response: {:?}", res))?;

    let formatted_tool_calls: Vec<Value> = tool_calls
        .iter()
        .map(|tool_call| {
            let mut tool_call_json = json!({
                "name": tool_call.name,
            });

            let arguments_json = serde_json::to_value(&tool_call.arguments).unwrap();
            tool_call_json["input"] = arguments_json;
            tool_call_json
        })
        .collect();

    let json_output = serde_json::to_string_pretty(&formatted_tool_calls)?;
    Ok(json_output)
}

#[plugin_fn]
pub fn models() -> FnResult<String> {
    let models_vec: Vec<Model> = MODELS.to_vec();
    let models_json = serde_json::to_string(&models_vec)?;
    info!("Returning models");
    Ok(models_json)
}
