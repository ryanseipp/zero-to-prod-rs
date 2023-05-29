use std::fmt::Write;

use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;

pub async fn admin_newsletter(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Admin newsletter</title>
    </head>
    <body>
        {msg_html}
        <form name="sendNewsletter" action="/admin/newsletter" method="post">
            <label>
                Title
                <input name="title" placeholder="Newsletter title" />
            </label>
            <label>
                HTML Content
                <input name="content_html" placeholder="Newsletter HTML Content" />
            </label>
            <label>
                Text Content
                <input name="content_text" placeholder="Newsletter Text Content" />
            </label>
        </form>
    </body>
</html>"#,
        )))
}
