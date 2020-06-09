use actix_web::client::Client;
use actix_web::HttpResponse;
use http::Uri;
use mediaproxy_common::query::Query;

fn replace_path_and_query(original: &Uri, new_path_and_query: String) -> Result<Uri, http::Error> {
    Uri::builder()
        .scheme(original.scheme().map(|a| a.as_str()).unwrap_or(""))
        .authority(original.authority().map(|a| a.as_str()).unwrap_or(""))
        .path_and_query(new_path_and_query.as_str())
        .build()
}

pub async fn forward(query: Query, forward_uri: Uri) -> Result<HttpResponse, actix_web::Error> {
    let fingerprint = query.to_fingerprint();

    let new_uri = replace_path_and_query(&forward_uri, format!("/{}", fingerprint)).unwrap();

    let res = Client::default().get(new_uri).send().await?;

    let mut client_resp = HttpResponse::build(res.status());

    let stream = actix_web::body::BodyStream::new(res);

    Ok(client_resp.body(stream))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_path() {
        let original_uri = "https://github.com/ThePicoNerd/mediaproxy-server"
            .parse::<Uri>()
            .unwrap();
        let replaced_uri =
            replace_path_and_query(&original_uri, "/ThePicoNerd/mediaproxy-router".to_string())
                .unwrap();
        assert_eq!(
            replaced_uri.to_string(),
            "https://github.com/ThePicoNerd/mediaproxy-router"
        );
    }
}
