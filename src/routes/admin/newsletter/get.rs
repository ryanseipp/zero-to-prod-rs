use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

pub async fn admin_newsletter() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Admin newsletter</title>
    </head>
    <body>
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
    ))
}
