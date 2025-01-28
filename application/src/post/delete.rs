use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use domain::models::Post;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn delete_post(post_id: i32) -> Result<Vec<Post>, NotFound<String>> {
    use domain::schema::posts;
    use domain::schema::posts::dsl::*;

    let response: Response;

    let num_delete =
        match diesel::delete(posts.filter(id.eq(post_id))).execute(&mut establish_connection()) {
            Ok(count) => count,
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    let response = Response {
                        body: ResponseBody::Message(format!(
                            "Error deleting post with id {} - {}",
                            post_id, err
                        )),
                    };
                    return Err(NotFound(serde_json::to_string(&response).unwrap()));
                }
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        };

    if num_delete > 0 {
        match posts::table
            .select(posts::all_columns)
            .load::<Post>(&mut establish_connection())
        {
            Ok(mut posts_) => {
                posts_.sort();
                Ok(posts_)
            }
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        }
    } else {
        response = Response {
            body: ResponseBody::Message(format!("Error no post with id {}", post_id)),
        };
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    }
}
