use super::{ToolExector, ToolPrompt};
use anyhow::Result;
use std::{
    fmt::{self, Debug},
    path::Path,
};
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Default)]
pub struct FileAppend {
    filename: String,
    content: String,
}

impl FileAppend {
    pub fn new(filename: String, content: String) -> Self {
        FileAppend { filename, content }
    }
}

impl ToolExector for FileAppend {
    async fn execute(&self) -> Result<String> {
        let path = Path::new(".").join("output").join(&self.filename);
        let mut file = File::options().append(true).open(path).await?;
        file.write_all(self.content.as_bytes()).await?;
        Ok("写入成功".to_string())
    }
}

impl ToolPrompt for FileAppend {
    fn command(&self) -> String {
        r#"{
            "name": "file_append",
            "description": "文件写入工具：用于将内容追加到文件末尾",
            "args": [
                {
                    "name": "filename",
                    "type": "string",
                    "description": "文件名称，要使用曾经创建过的文件"
                },
                {
                    "name": "content",
                    "type": "string",
                    "description": "文件追加内容"
                }
            ]
        }"#
        .to_string()
    }

    fn resource(&self) -> String {
        "将内容追加到文件末尾".to_string()
    }
}

impl Debug for FileAppend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileAppend")
            .field("filename", &self.filename)
            .field("content", &"...")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_append() -> Result<()> {
        let file_write = FileAppend::new("test.txt".to_string(), "test content 1".to_string());
        let result = file_write.execute().await?;
        assert_eq!(result, "写入成功");

        Ok(())
    }
}
