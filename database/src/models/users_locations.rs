use crate::models::location::Location;
use crate::models::user::User;
use crate::schema::users_locations;

use diesel::prelude::*;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Location))]
#[diesel(table_name = users_locations)]
#[diesel(primary_key(user_id, location_id))]
pub struct UserLocation {
    pub user_id: i32,
    pub location_id: i32,
}
