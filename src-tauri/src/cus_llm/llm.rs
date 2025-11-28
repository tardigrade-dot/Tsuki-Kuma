use std::path::PathBuf;

use tauri::command;

use super::qwen_llm::qwen_infer;
use crate::cus_llm::LLM_MODEL_BASE_PATH;

#[command]
pub fn llm_infer(prompt: &str) -> Result<String, String> {
    // 1. 使用 format! 宏
    let chat_prompt = format!("{}<|im_end|>", prompt);
    
    let path_buf = LLM_MODEL_BASE_PATH
        .get()
        .ok_or_else(|| String::from("LLM_MODEL_BASE_PATH 尚未初始化"))?;
    
    // 2. 将 &PathBuf 转换为 &str
    //    PathBuf 实现了 Deref<Target=Path>，因此 path_buf 相当于 &Path
    let path_str = path_buf
        .to_str()
        .ok_or_else(|| String::from("路径包含非法的 UTF-8 字符"))?;
    let model_path = path_str;
    
    // 2. 调用 qwen_infer 并使用 ? 处理 Result
    let reply = qwen_infer(
        model_path,
        // "/Users/larry/github.com/Tsuki-Kuma/src-tauri/ai_models/Qwen3-0.6B",
        &chat_prompt, // 注意这里应该传递 &str，但 qwen_infer 接收 &str，这里是 OK 的
        false,
        None, None, None, None, None, None
    )
    // 3. 将 qwen_infer 返回的 Result<String, anyhow::Error> 
    //    转换为 Result<String, String>，这是 tauri 友好的格式
    .map_err(|e| e.to_string()); // 将 anyhow::Error 转换为 String 错误信息

    // 4. 返回 Result<String, String>
    println!("模型回复: {:?}", reply);
    reply
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_llm() {

        let prompt = "介绍一下你自己.";
        println!("{:?}", llm_infer(prompt));
    }

    #[test]
    fn test_tts() {

        let system_prompt = "You are a helpful assistant.";
        // let chat_prompt = format!(
        //     "<|im_start|>system\n{}\n<|im_end|>\n<|im_start|>user\n{}\n<|im_end|>\n<|im_start|>assistant\n",
        //     system_prompt,
        //     "你好, 介绍一下你自己."
        // );

        let chat_prompt = "简单介绍你自己<|im_end|>";
        let reply = qwen_infer(
            "/Users/larry/github.com/Tsuki-Kuma/src-tauri/ai_models/Qwen3-0.6B",
            chat_prompt,
            false,
            None, None, None, None, None,None
        );
        println!("模型回复: {}", reply.unwrap());
    }
}
