use super::{ToolExector, ToolPrompt};
use anyhow::Result;
use std::{
    fmt::{self, Debug},
    path::Path,
};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

#[derive(Default)]
pub struct FileWrite {
    filename: String,
    content: String,
}

impl FileWrite {
    pub fn new(filename: String, content: String) -> Self {
        FileWrite { filename, content }
    }
}

impl ToolExector for FileWrite {
    async fn execute(&self) -> Result<String> {
        let path = Path::new(".").join("output");
        fs::create_dir_all(&path).await?;
        let mut file = File::create(path.join(&self.filename)).await?;
        file.write_all(self.content.as_bytes()).await?;
        Ok("写入成功".to_string())
    }
}

impl ToolPrompt for FileWrite {
    fn command(&self) -> String {
        r#"{
            "name": "file_write",
            "description": "文件写入工具：用于将内容写入文件，将覆盖原有内容",
            "args": [
                {
                    "name": "filename",
                    "type": "string",
                    "description": "文件名称"
                },
                {
                    "name": "content",
                    "type": "string",
                    "description": "文件内容"
                }
            ]
        }"#
        .to_string()
    }

    fn resource(&self) -> String {
        "将内容写入文件".to_string()
    }
}

impl Debug for FileWrite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileWrite")
            .field("filename", &self.filename)
            .field("content", &self.content)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_write() {
        let file_write = FileWrite::new("test.txt".to_string(), "test content".to_string());
        let result = file_write.execute().await.unwrap();
        assert_eq!(result, "写入成功");
    }
}
