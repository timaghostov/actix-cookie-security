# actix-cookie-security


```
#[secured(session, [Role::Editor, Role::Admin])]
#[get("/editor_admin_handle")]
async fn editor_admin_handle(
    session: Session,
) -> Result<HttpResponse, ExampleCustomError> {
    Ok(HttpResponse::Ok().finish())
}
```

see more examples