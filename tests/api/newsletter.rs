use uuid::Uuid;
use wiremock::{
    matchers::{any, method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::{assert_is_redirect_to, spawn_app, ConfirmationLinks, TestApp};

#[derive(serde::Serialize)]
pub struct Newsletter {
    pub title: Option<String>,
    pub content_html: Option<String>,
    pub content_text: Option<String>,
    pub idempotency_key: Option<Uuid>,
}

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    let app = spawn_app().await;
    app.post_login_default().await;
    create_unconfirmed_subscriber(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    let newsletter_request = Newsletter {
        title: Some("Newsletter title".to_string()),
        content_html: Some("<p>Newsletter body as HTML</p>".to_string()),
        content_text: Some("Newsletter body as plain text".to_string()),
        idempotency_key: Some(Uuid::new_v4()),
    };

    let response = app.post_newsletters(&newsletter_request).await;
    assert_is_redirect_to(&response, "/admin/newsletter");

    let html_page = app.get_newsletters_html().await;
    assert!(html_page.contains(r#"<p><i>The newsletter issue has been published!</i></p>"#))
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    let app = spawn_app().await;
    app.post_login_default().await;
    create_confirmed_subscriber(&app).await;

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_request = Newsletter {
        title: Some("Newsletter title".to_string()),
        content_html: Some("<p>Newsletter body as HTML</p>".to_string()),
        content_text: Some("Newsletter body as plain text".to_string()),
        idempotency_key: Some(Uuid::new_v4()),
    };

    let response = app.post_newsletters(&newsletter_request).await;
    assert_is_redirect_to(&response, "/admin/newsletter");

    let html_page = app.get_newsletters_html().await;
    assert!(html_page.contains(r#"<p><i>The newsletter issue has been published!</i></p>"#))
}

#[tokio::test]
async fn newsletters_returns_400_for_invalid_data() {
    let app = spawn_app().await;
    app.post_login_default().await;

    let test_cases = vec![
        (
            Newsletter {
                title: None,
                content_html: Some("<p>Newsletter body as HTML</p>".to_string()),
                content_text: Some("Newsletter body as plain text".to_string()),
                idempotency_key: Some(Uuid::new_v4()),
            },
            "missing title",
        ),
        (
            Newsletter {
                title: Some("Newsletter title".to_string()),
                content_html: None,
                content_text: None,
                idempotency_key: Some(Uuid::new_v4()),
            },
            "missing content",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_newsletters(&invalid_body).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn requests_missing_authorization_are_redirected_to_login() {
    let app = spawn_app().await;

    let newsletter_request = Newsletter {
        title: Some("Newsletter title".to_string()),
        content_html: Some("<p>Newsletter body as HTML</p>".to_string()),
        content_text: Some("Newsletter body as plain text".to_string()),
        idempotency_key: Some(Uuid::new_v4()),
    };

    let response = app.post_newsletters(&newsletter_request).await;
    assert_is_redirect_to(&response, "/login");
}

// #[tokio::test]
// async fn newsletter_creation_is_idempotent() {
//     let app = spawn_app().await;
//     create_confirmed_subscriber(&app).await;
//     app.post_login_default().await;
//
//     Mock::given(path("/email"))
//         .and(method("POST"))
//         .respond_with(ResponseTemplate::new(200))
//         .expect(1)
//         .mount(&app.email_server)
//         .await;
//
//     let newsletter_request = Newsletter {
//         title: Some("Newsletter title".into()),
//         content_html: Some("<p>Newsletter body as HTML</p>".into()),
//         content_text: Some("Newsletter body as plain text".into()),
//         idempotency_key: Some(Uuid::new_v4()),
//     };
//
//     let response = app.post_newsletters(&newsletter_request).await;
//     assert_is_redirect_to(&response, "/admin/newsletter");
//
//     let html_page = app.get_newsletters_html().await;
//     assert!(html_page.contains(r#"<p><i>The newsletter issue has been published!</i></p>"#));
//
//     let response = app.post_newsletters(&newsletter_request).await;
//     assert_is_redirect_to(&response, "/admin/newsletter");
//
//     let html_page = app.get_newsletters_html().await;
//     assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));
// }

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    app.post_subscriptions(body.into())
        .await
        .error_for_status()
        .unwrap();

    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();

    app.get_confirmation_links(email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
    let confirmation_link = create_unconfirmed_subscriber(app).await;
    reqwest::get(confirmation_link.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}
