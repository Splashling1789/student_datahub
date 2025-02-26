//! Database models and implementations.

#[doc(hidden)]
mod entry;
#[doc(hidden)]
mod period;
#[doc(hidden)]
mod subject;

use diesel::{Associations, Queryable, Selectable};
use diesel::internal::derives::multiconnection::chrono::NaiveDate;

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::entry)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Subject))]
/// Model for the dedicated time to a specific subject in a specific date.
pub struct Entry {
    pub id: i32,
    pub date: NaiveDate,
    pub subject_id: i32,
    pub dedicated_time: i32,
}

#[derive(Selectable, Queryable, Clone, Debug)]
#[diesel(table_name = crate::schema::periods)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
/// Model for a study period (semester or similar)
pub struct Period {
    pub id: i32,
    pub initial_date: NaiveDate,
    pub final_date: NaiveDate,
    pub description: String,
}

#[derive(Queryable, Selectable, Associations, Clone, Debug)]
#[diesel(table_name = crate::schema::subjects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Period))]
/// Model for a subject in a specific subject.
pub struct Subject {
    pub id: i32,
    pub period_id: i32,
    pub short_name: String,
    pub name: String,
    pub final_score: Option<f32>,
}