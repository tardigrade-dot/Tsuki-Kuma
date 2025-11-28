mod commands;
mod cus_tts;
mod cus_llm;

use commands::default::{read, write};
use commands::maths::add;
use commands::tts::generate_voice;
use cus_llm::llm::llm_infer;
use tauri::Manager;
use anyhow::Result;
use tauri::AppHandle;
use tauri::path::BaseDirectory;

use crate::cus_tts::TTS_MODEL_BASE_PATH;
use crate::cus_llm::LLM_MODEL_BASE_PATH;

const TTS_MODEL_RELATIVE_PATH: &str = "ai_models/supertonic";
const LLM_MODEL_RELATIVE_PATH: &str = "ai_models/Qwen3-0.6B";

#[allow(clippy::missing_panics_doc)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {

            let result = initialize_model_paths(app.handle());
            if let Err(e) = result {
                eprintln!("初始化模型路径失败: {:?}", e);
                // 这里可以选择让程序失败或继续，取决于错误是否致命
            }

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read, write, add, generate_voice, llm_infer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


fn initialize_model_paths(app_handle: &AppHandle) -> Result<()> {
    
    let tts_model_base_dir = app_handle.path().resolve(TTS_MODEL_RELATIVE_PATH, BaseDirectory::Resource)?;
    if !tts_model_base_dir.exists() {
        // 在开发模式下，这可能需要更复杂的处理，但在构建后应始终存在
        anyhow::bail!("模型基目录未找到: {}", tts_model_base_dir.display());
    }
    println!("模型基目录已找到: {}", tts_model_base_dir.display());
    if TTS_MODEL_BASE_PATH.set(tts_model_base_dir).is_err() {
        anyhow::bail!("尝试重复初始化 ONNX_BASE_PATH");
    }

    let llm_model_base_dir = app_handle.path().resolve(LLM_MODEL_RELATIVE_PATH, BaseDirectory::Resource)?;
    if !llm_model_base_dir.exists() {
        // 在开发模式下，这可能需要更复杂的处理，但在构建后应始终存在
        anyhow::bail!("模型基目录未找到: {}", llm_model_base_dir.display());
    }
    println!("模型基目录已找到: {}", llm_model_base_dir.display());
    if LLM_MODEL_BASE_PATH.set(llm_model_base_dir).is_err() {
        anyhow::bail!("尝试重复初始化 ONNX_BASE_PATH");
    }

    Ok(())
}