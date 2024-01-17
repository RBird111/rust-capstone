use crate::schema::locations;

use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, AsChangeset, Selectable, Identifiable, Serialize, Debug, Clone, Deserialize,
)]
#[diesel(table_name = locations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Location {
    pub id: i32,
    pub address: String,
    pub city: String,
    pub state: String,
    pub lat: Option<BigDecimal>,
    pub lng: Option<BigDecimal>,
}

#[derive(Insertable, Debug, Clone, Deserialize, Serialize)]
#[diesel(table_name = locations)]
pub struct LocationForm {
    pub address: String,
    pub city: String,
    pub state: String,
    pub lat: Option<BigDecimal>,
    pub lng: Option<BigDecimal>,
}
