use super::DataResult;
use crate::models::review::*;
use crate::models::user::User;
use crate::schema::reviews::dsl::*;

use diesel::prelude::*;
use diesel::PgConnection;
use rand::seq::SliceRandom;
use serde_json::{json, Value};

pub fn get_all_reviews(conn: &mut PgConnection) -> DataResult<Value> {
    let review_data: Vec<_> = reviews.select(Review::as_select()).load(conn)?;

    Ok(json!({
        "reviews": review_data
        .into_iter()
        .filter_map(|r| r.eager_load(conn).ok())
        .collect::<Vec<Value>>()
    }))
}

pub fn get_review_by_id(conn: &mut PgConnection, review_id: i32) -> DataResult<Value> {
    Ok(json!({
        "review": reviews
        .find(review_id)
        .first::<Review>(conn)?
        .eager_load(conn)?
    }))
}

pub fn get_random_reviews(conn: &mut PgConnection, num: usize) -> DataResult<Value> {
    let mut rng = rand::thread_rng();

    let all_reviews: Vec<Review> = reviews.select(Review::as_select()).load(conn)?;

    Ok(json!({
        "reviews": all_reviews
        .choose_multiple(&mut rng, num)
        .cloned()
        .filter_map(|r| r.eager_load(conn).ok())
        .collect::<Vec<Value>>()
    }))
}

pub fn create_new_review(conn: &mut PgConnection, review: ReviewForm) -> DataResult<Value> {
    let new_review: Review = diesel::insert_into(reviews)
        .values(review)
        .get_result(conn)?;

    Ok(json!({
        "review": new_review.eager_load(conn)?
    }))
}

pub fn update_review(conn: &mut PgConnection, review: Review) -> DataResult<Value> {
    let updated_review: Review = diesel::update(reviews).set(review).get_result(conn)?;

    Ok(json!({
        "review": updated_review.eager_load(conn)?
    }))
}

pub fn delete_review(conn: &mut PgConnection, review_id: i32) -> Value {
    let success = r#"{"message": "Review successfully deleted"}"#;
    let error = r#"{"message": "Unable to locate review"}"#;

    match diesel::delete(reviews.find(review_id)).execute(conn) {
        Ok(0) => serde_json::from_str(error),
        Ok(_) => serde_json::from_str(success),
        Err(_) => serde_json::from_str(error),
    }
    .expect("error serializing message")
}

pub fn get_user_reviews(conn: &mut PgConnection, curr_user: User) -> DataResult<Value> {
    let user_reviews: Vec<Review> = reviews
        .select(Review::as_select())
        .filter(user_id.eq(curr_user.id))
        .load(conn)?;

    Ok(json!({
        "reviews": user_reviews
        .into_iter()
        .filter_map(|r| r.eager_load(conn).ok())
        .collect::<Vec<Value>>()
    }))
}
