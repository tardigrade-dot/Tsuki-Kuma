pub mod llm;
mod llm_candle;
mod tokenizer_stream;
mod qwen_llm;

use once_cell::sync::OnceCell;
use std::path::PathBuf;

pub static LLM_MODEL_BASE_PATH: OnceCell<PathBuf> = OnceCell::new();