use crate::schema::periods::dsl::periods;
use crate::schema::periods::{final_date, initial_date};
use diesel::deserialize::FromSqlRow;
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use diesel::prelude::*;
use diesel::row::Row;
use diesel::sqlite::Sqlite;

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::entry)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Subject))]
pub struct Entry {
    pub id: i32,
    pub date: NaiveDate,
    pub subject_id: i32,
    pub dedicated_time: i32,
}

impl Entry {
    pub fn get_period(&self, conn: &mut SqliteConnection) -> Option<Period> {
        periods
            .filter(initial_date.le(&self.date))
            .filter(final_date.ge(&self.date))
            .load::<Period>(conn)
            .expect("Error loading period")
            .pop()
    }
}

#[derive(Selectable, Queryable, Clone, Debug)]
#[diesel(table_name = crate::schema::periods)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Period {
    pub id: i32,
    pub initial_date: NaiveDate,
    pub final_date: NaiveDate,
    pub description: String,
}

impl Period {
    pub fn to_string(&self) -> String {
        format!(
            "{} - {}\t{} (ID:{})",
            self.initial_date
                .format(crate::plan::period::FORMAT)
                .to_string(),
            self.final_date
                .format(crate::plan::period::FORMAT)
                .to_string(),
            self.description.to_string(),
            self.id
        )
    }
}

#[derive(Queryable, Selectable, Associations, Clone, Debug)]
#[diesel(table_name = crate::schema::subjects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Period))]
pub struct Subject {
    pub id: i32,
    pub period_id: i32,
    pub short_name: String,
    pub name: String,
    pub final_score: Option<f32>,
}
