use std::path::Path;

use uuid::Uuid;

/// Simple builder for multipart/form-data test
///
/// # Examples
///
/// ```
/// #[cfg(test)]
/// mod tests {
///     use actix_multipart_test::MultiPartFormDataBuilder;
///     use actix_web::{test, App};
///     use super::*;
///
///     #[actix_web::test]
///     async fn test_should_upload_file() {
///
///         let mut app =
///             test::init_service(
///                     App::new()
///                     .service(yourmultipartformhandler)
///                 )
///                 .await;
///
///         let mut multipart_form_data_builder = MultiPartFormDataBuilder::new();
///         multipart_form_data_builder.with_file("tests/sample.png", "sample", "image/png", "sample.png");
///         multipart_form_data_builder.with_text("name", "some_name");
///         let (header, body) = multipart_form_data_builder.build();
///
///
///         let req = test::TestRequest::post()
///             .uri("/someurl")
///             .insert_header(header)
///             .set_payload(body)
///             .to_request();
///         let resp = test::call_service(&mut app, req).await;
///
///         assert!(resp.status().is_success());
///
///     }
/// }
/// ```
pub struct MultiPartFormDataBuilder {
    files: Vec<(String, String, String, Box<dyn AsRef<Path>>)>,
    texts: Vec<(String, String, String)>,
}

impl MultiPartFormDataBuilder {
    /// Create new MultiPartFormDataBuilder
    pub fn new() -> MultiPartFormDataBuilder {
        MultiPartFormDataBuilder {
            files: vec![],
            texts: vec![],
        }
    }

    /// Add text to multipart/form-data
    ///
    /// name is form name
    ///
    /// value is form value
    ///
    /// Returns &mut MultiPartFormDataBuilder
    pub fn with_text(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> &mut MultiPartFormDataBuilder {
        self.texts
            .push((name.into(), value.into(), "text/plain".to_string()));
        self
    }

    /// Add file to multipart/form-data
    ///
    /// path is file path
    ///
    /// name is form name
    ///
    /// content_type is file content type
    ///
    /// file_name is file name
    pub fn with_file(
        &mut self,
        path: impl AsRef<Path> + 'static,
        name: impl Into<String>,
        content_type: impl Into<String>,
        file_name: impl Into<String>,
    ) -> &mut MultiPartFormDataBuilder {
        self.files.push((
            name.into(),
            file_name.into(),
            content_type.into(),
            Box::new(path),
        ));
        self
    }

    /// Build multipart/form-data
    ///
    /// Returns ((header_name, header_value), body)
    ///
    /// header_name is "Content-Type"
    ///
    /// header_value is "multipart/form-data; boundary=..."
    ///
    /// body is binary data
    pub fn build(&self) -> ((String, String), Vec<u8>) {
        let boundary = Uuid::new_v4().to_string();

        let mut body = vec![];

        for file in self.files.iter() {
            body.extend(format!("--{}\r\n", boundary).as_bytes());
            body.extend(
                format!(
                    "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                    file.0, file.1
                )
                .as_bytes(),
            );
            body.extend(format!("Content-Type: {}\r\n", file.2).as_bytes());
            let data = std::fs::read(file.3.as_ref()).unwrap();
            body.extend(format!("Content-Length: {}\r\n\r\n", data.len()).as_bytes());
            body.extend(data);
            body.extend("\r\n".as_bytes());
        }

        for text in self.texts.iter() {
            body.extend(format!("--{}\r\n", boundary).as_bytes());
            body.extend(
                format!("Content-Disposition: form-data; name=\"{}\"\r\n", text.0).as_bytes(),
            );
            body.extend(format!("Content-Type: {}\r\n", text.2).as_bytes());
            let data = text.1.as_bytes();
            body.extend(format!("Content-Length: {}\r\n\r\n", data.len()).as_bytes());
            body.extend(data);
            body.extend("\r\n".as_bytes());
        }

        body.extend(format!("--{}--\r\n", boundary).as_bytes());

        let header_value = format!("multipart/form-data; boundary={}", boundary);
        let header = ("Content-Type".to_string(), header_value);

        (header, body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_build_multipart_form_with_text() {
        let mut multipart_form_data_builder = MultiPartFormDataBuilder::new();
        multipart_form_data_builder.with_file(
            "tests/sample.png",
            "sample",
            "image/png",
            "sample.png",
        );
        multipart_form_data_builder.with_text("name", "some_name");
        let (header, body) = multipart_form_data_builder.build();

        assert_eq!(header.0, "Content-Type");
        assert!(header.1.starts_with("multipart/form-data; boundary="));
        assert!(body.len() > 0);
    }
}
