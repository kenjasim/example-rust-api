use warp::Filter;

// A function to build our routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_todo()
}

fn get_todo() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todos" / i32)
        .and(warp::get())
        .and_then(super::handlers::get_todo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::StatusCode;

    #[tokio::test]
    async fn test_get_todo() {
        let api = routes();

        let resp = warp::test::request()
            .method("GET")
            .path("/todos/1")
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
