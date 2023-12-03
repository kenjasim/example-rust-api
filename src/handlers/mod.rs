use super::domain;

pub async fn get_todo(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let todo = domain::Todo {
        id,
        title: "Example todo".to_string(),
        completed: false,
    };
    
    Ok(warp::reply::json(&todo))
}