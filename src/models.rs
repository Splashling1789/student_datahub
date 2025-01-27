use diesel::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::deserialize::FromSqlRow;
use diesel::row::{Field, Row};
use diesel::sqlite::Sqlite;

pub struct DateTime(pub NaiveDate);

impl FromSqlRow<diesel::sql_types::Text, Sqlite> for DateTime {
    fn build_from_row<'a>(row: &impl Row<'a, Sqlite>) -> diesel::deserialize::Result<Self> {
        let text : String = row.get_value::<diesel::sql_types::Text, String, _>(0)?;
        NaiveDate::parse_from_str(&text, "%d-%m-%Y")
            .map(|d| DateTime(d))
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::entry)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Subject))]
pub struct Entry {
    pub id: i32,
    pub date: DateTime,
    pub subject_id: i32,
    pub dedicated_time: i32
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::periods)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Period {
    pub id: i32,
    pub initial_date: DateTime,
    pub final_date: DateTime,
    pub description: String
}

#[derive(Queryable, Selectable, Associations)]
#[diesel(table_name = crate::schema::subjects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Period))]
pub struct Subject {
    pub id: i32,
    pub period_id: i32,
    pub short_name: String,
    pub name: String,
    pub final_score: Option<f32>
}