use super::DataResult;
use crate::models::business::Business;
use crate::models::image::Image;
use crate::models::review::*;
use crate::models::user::User;
use crate::schema::reviews::dsl::*;
use crate::schema::{businesses, images, users};

use diesel::prelude::*;
use diesel::PgConnection;
use rand::seq::SliceRandom;
use serde_json::{json, to_value, Value};

pub fn get_all_reviews(conn: &mut PgConnection) -> DataResult<Vec<ReviewFull>> {
    let review_data: Vec<(Review, User, Business, Image)> = reviews
        .inner_join(users::table)
        .inner_join(businesses::table)
        .inner_join(images::table)
        .select((
            Review::as_select(),
            User::as_select(),
            Business::as_select(),
            Image::as_select(),
        ))
        .load(conn)?;

    Ok(review_data
        .into_iter()
        .map(|(review, user, business, image)| ReviewFull {
            review,
            user,
            business,
            images: vec![image]
        })
        .collect())
}

pub fn get_review_by_id(conn: &mut PgConnection, review_id: i32) -> DataResult<Review> {
    Ok(reviews.find(review_id).first(conn)?)
}

pub fn get_random_reviews(conn: &mut PgConnection, num: usize) -> DataResult<Value> {
    let mut rng = rand::thread_rng();
    let all_reviews = get_all_reviews(conn)?;
    let rand_reviews: Vec<ReviewFull> = all_reviews
        .choose_multiple(&mut rng, num)
        .cloned()
        .collect();
    let res: Vec<Value> = rand_reviews
        .into_iter()
        .filter_map(|r| to_value(&r).ok())
        .collect();

    Ok(json!({
        "reviews": res
    }))
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
