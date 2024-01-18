use super::DataResult;
use crate::models::image::*;
use crate::models::user::User;
use crate::schema::images::dsl::*;

use diesel::prelude::*;
use diesel::PgConnection;
use serde_json::Value;

pub fn get_all_images(conn: &mut PgConnection) -> DataResult<Vec<Image>> {
    Ok(images.select(Image::as_select()).load(conn)?)
}

pub fn get_image_by_id(conn: &mut PgConnection, image_id: i32) -> DataResult<Image> {
    Ok(images.find(image_id).first(conn)?)
}

pub fn get_user_images(conn: &mut PgConnection, curr_user: User) -> DataResult<Vec<Image>> {
    Ok(Image::belonging_to(&curr_user)
        .select(Image::as_select())
        .load(conn)?)
}

pub fn upload_image(conn: &mut PgConnection, image: ImageForm) -> DataResult<Image> {
    Ok(diesel::insert_into(images).values(image).get_result(conn)?)
}

pub fn delete_image(conn: &mut PgConnection, image_id: i32) -> Value {
    let success = r#"{"message": "Image successfully deleted"}"#;
    let error = r#"{"message": "Unable to locate image"}"#;

    match diesel::delete(images.find(image_id)).execute(conn) {
        Ok(0) => serde_json::from_str(error),
        Ok(_) => serde_json::from_str(success),
        Err(_) => serde_json::from_str(error),
    }
    .expect("error serializing message")
}
