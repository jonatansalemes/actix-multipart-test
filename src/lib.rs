use std::path::Path;

use uuid::Uuid;

pub struct MultiPartFormDataBuilder {
    files: Vec<(String, String, String, Box<dyn AsRef<Path>>)>,
    texts: Vec<(String, String, String)>,
}

impl MultiPartFormDataBuilder {
    pub fn new() -> MultiPartFormDataBuilder {
        MultiPartFormDataBuilder {
            files: vec![],
            texts: vec![],
        }
    }

    pub fn with_text(&mut self,
                     name: impl Into<String>,
                     value: impl Into<String>) -> &mut MultiPartFormDataBuilder {
        self.texts.push((name.into(), value.into(), "text/plain".to_string()));
        self
    }

    pub fn with_file(&mut self,
                     path: impl AsRef<Path> + 'static,
                     name: impl Into<String>,
                     content_type: impl Into<String>,
                     file_name: impl Into<String>) -> &mut MultiPartFormDataBuilder {
        self.files.push((name.into(), file_name.into(), content_type.into(), Box::new(path)));
        self
    }

    pub fn build(&self) -> ((String, String), Vec<u8>) {
        let boundary = Uuid::new_v4().to_string();

        let mut body = vec![];

        for file in self.files.iter() {
            body.extend(format!("--{}\r\n", boundary).as_bytes());
            body.extend(format!("Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n", file.0, file.1).as_bytes());
            body.extend(format!("Content-Type: {}\r\n", file.2).as_bytes());
            let data = std::fs::read(file.3.as_ref()).unwrap();
            body.extend(format!("Content-Length: {}\r\n\r\n", data.len()).as_bytes());
            body.extend(data);
            body.extend("\r\n".as_bytes());
        }

        for text in self.texts.iter() {
            body.extend(format!("--{}\r\n", boundary).as_bytes());
            body.extend(format!("Content-Disposition: form-data; name=\"{}\"\r\n", text.0).as_bytes());
            body.extend(format!("Content-Type: {}\r\n", text.2).as_bytes());
            body.extend(format!("Content-Length: {}\r\n\r\n", text.1.len()).as_bytes());
            body.extend(text.1.as_bytes());
            body.extend("\r\n".as_bytes());
        }

        body.extend(format!("--{}\r\n", boundary).as_bytes());

        let header_value = format!("multipart/form-data; boundary={}", boundary);
        let header = ("Content-Type".to_string(), header_value);

        (header, body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_build_multipart() {
        let mut multipart_builder = MultiPartFormDataBuilder::new();
        multipart_builder.with_file("tests/sample.png", "sample", "image/png", "sample.png");
        multipart_builder.with_text("name", "some_name");
        let (header, body) = multipart_builder.build();

        assert_eq!(header.0, "Content-Type");
        assert!(header.1.starts_with("multipart/form-data; boundary="));
        assert!(body.len() > 0);
    }
}
