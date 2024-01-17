use super::DataResult;
use crate::models::review::*;
use crate::models::user::User;
use crate::schema::reviews::dsl::*;

use diesel::prelude::*;
use diesel::PgConnection;
use rand::seq::SliceRandom;
use serde_json::Value;

pub fn get_all_reviews(conn: &mut PgConnection) -> DataResult<Vec<Review>> {
    Ok(reviews.select(Review::as_select()).load(conn)?)
}

pub fn get_review_by_id(conn: &mut PgConnection, review_id: i32) -> DataResult<Review> {
    Ok(reviews.find(review_id).first(conn)?)
}

pub fn get_random_reviews(conn: &mut PgConnection, num: usize) -> DataResult<Vec<Review>> {
    let mut rng = rand::thread_rng();
    let all_reviews = get_all_reviews(conn)?;
    Ok(all_reviews
        .choose_multiple(&mut rng, num)
        .cloned()
        .collect())
}

pub fn create_new_review(conn: &mut PgConnection, review: ReviewForm) -> DataResult<Review> {
    Ok(diesel::insert_into(reviews)
        .values(review)
        .get_result(conn)?)
}

pub fn update_review(conn: &mut PgConnection, review: Review) -> DataResult<Review> {
    Ok(diesel::update(reviews).set(review).get_result(conn)?)
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

pub fn get_user_reviews(conn: &mut PgConnection, curr_user: User) -> DataResult<Vec<Review>> {
    Ok(reviews
        .select(Review::as_select())
        .filter(user_id.eq(curr_user.id))
        .load(conn)?)
}
