use diesel::prelude::*;
use serde_json::Value;

use super::DataResult;
use crate::models::image::*;
use crate::models::user::User;
use crate::schema::images::dsl::*;

pub fn get_all_images(conn: &mut PgConnection) -> DataResult<Value> {
    let image_data: ImageArray = ImageArray::new(images.select(Image::as_select()).load(conn)?);
    Ok(image_data.eager_load(conn)?)
}

pub fn get_image_by_id(conn: &mut PgConnection, image_id: i32) -> DataResult<Value> {
    let image: Image = images.find(image_id).first(conn)?;
    Ok(image.eager_load(conn)?)
}

pub fn get_user_images(conn: &mut PgConnection, curr_user: User) -> DataResult<Value> {
    let image_data: ImageArray = ImageArray::new(
        Image::belonging_to(&curr_user)
            .select(Image::as_select())
            .load(conn)?,
    );

    Ok(image_data.eager_load(conn)?)
}

pub fn upload_image(conn: &mut PgConnection, image: ImageForm) -> DataResult<Value> {
    let new_image: Image = diesel::insert_into(images).values(image).get_result(conn)?;
    Ok(new_image.eager_load(conn)?)
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
