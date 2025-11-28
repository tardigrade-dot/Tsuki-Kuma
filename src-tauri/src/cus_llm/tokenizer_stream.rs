
use anyhow::Result;
use tokenizers::Tokenizer;

/// 简单的 TokenOutputStream，可用于增量解码
pub struct TokenOutputStream {
    tokenizer: Tokenizer,
    prev: Vec<u32>,
}

impl TokenOutputStream {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self {
            tokenizer,
            prev: vec![],
        }
    }

    pub fn tokenizer(&self) -> &Tokenizer {
        &self.tokenizer
    }

    pub fn clear(&mut self) {
        self.prev.clear();
    }

    pub fn next_token(&mut self, token: u32) -> Result<Option<String>> {
        self.prev.push(token);
        let out = self.tokenizer.decode(&self.prev, false).map_err(anyhow::Error::msg)?;
        Ok(Some(out))
    }

    /// 获取词元编号
    pub fn get_token(&self, s: &str) -> Option<u32> {
        self.tokenizer.token_to_id(s)
    }
}
