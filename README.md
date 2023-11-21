# Actix Multipart Form Data For Testing

This is a simple example of how to test multipart form data 
with actix-web endpoints.


## Usage

```
#[cfg(test)]
mod tests {

    use actix_multipart_test::MultiPartFormDataBuilder;
    use actix_web::{test, App};
    use super::*;

    #[actix_web::test]
    async fn test_should_upload_file() {
        
        let mut app =
            test::init_service(
                    App::new()
                    .service(yourmultipartformhandler)
                )
                .await;

        let mut multipart_form_data_builder = MultiPartFormDataBuilder::new();
        multipart_form_data_builder.with_file("tests/sample.png", "sample", "image/png", "sample.png");
        multipart_form_data_builder.with_text("name", "some_name");
        let (header, body) = multipart_form_data_builder.build();
        
        let req = test::TestRequest::post()
            .uri("/somerurl")
            .insert_header(header)
            .set_payload(body)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

    }
}
```
