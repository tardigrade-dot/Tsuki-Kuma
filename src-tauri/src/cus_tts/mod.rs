mod helper;
pub mod example_onnx;
use once_cell::sync::OnceCell;
use std::path::PathBuf;

pub static TTS_MODEL_BASE_PATH: OnceCell<PathBuf> = OnceCell::new();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tts() {
        
    }

}
    