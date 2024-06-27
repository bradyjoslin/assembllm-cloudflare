CF_ACCOUNT_ID := $(shell echo $$CF_ACCOUNT_ID)
WORKERS_AI_TOKEN := $(shell echo $$WORKERS_AI_TOKEN)

build:
	cargo build --release --target wasm32-unknown-unknown

test:
	extism call ./target/wasm32-unknown-unknown/release/assembllm_cloudflare.wasm models --log-level=info
	@extism call ./target/wasm32-unknown-unknown/release/assembllm_cloudflare.wasm completion \
		--set-config='{"api_key": "$(WORKERS_AI_TOKEN)", "account_id": "$(CF_ACCOUNT_ID)"}' \
		--input="Explain extism in the context of wasm succinctly." \
		--allow-host=api.cloudflare.com --log-level=info
	@extism call ./target/wasm32-unknown-unknown/release/assembllm_cloudflare.wasm completionWithTools \
		--set-config='{"api_key": "$(WORKERS_AI_TOKEN)", "account_id": "$(CF_ACCOUNT_ID)", "model": "@hf/nousresearch/hermes-2-pro-mistral-7b"}' \
		--input '{"tools": [{"name": "get_weather","description": "Get the current weather in a given location","input_schema": {"type": "object","properties": {"location": {"type": "string","description": "The city and state, e.g. San Francisco, CA"},"unit": {"type": "string","description": "The unit of temperature, always celsius"}},"required": ["location", "unit"]}}],"messages": [{"role": "user","content": "What is the weather like in San Francisco?"}]}' \
		--allow-host=api.cloudflare.com --log-level=info