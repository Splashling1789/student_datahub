use diesel::prelude::*;
use diesel::deserialize::FromSqlRow;
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use diesel::row::{Row};
use diesel::sqlite::Sqlite;
use crate::schema::periods::dsl::periods;
use crate::schema::periods::{final_date, initial_date};

#[derive(FromSqlRow)]
pub struct DateTime(pub NaiveDate);

impl FromSqlRow<diesel::sql_types::Text, Sqlite> for DateTime {
    fn build_from_row<'a>(row: &impl Row<'a, Sqlite>) -> diesel::deserialize::Result<Self> {
        let text : String = row.get_value::<diesel::sql_types::Text, String, _>(0)?;
        NaiveDate::parse_from_str(&text, "%m-%d-%Y")
            .map(|d| DateTime(d))
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
}

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::entry)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Subject))]
pub struct Entry {
    pub id: i32,
    pub date: NaiveDate,
    pub subject_id: i32,
    pub dedicated_time: i32
}

impl Entry {
    pub fn get_period(&self, conn: &mut SqliteConnection) -> Option<Period> {
        periods.filter(initial_date.le(&self.date)).filter(final_date.ge(&self.date)).load::<Period>(conn).expect("Error loading period").pop()
    }
}

#[derive(Selectable, Queryable, Clone, Debug)]
#[diesel(table_name = crate::schema::periods)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Period {
    pub id: i32,
    pub initial_date: NaiveDate,
    pub final_date: NaiveDate,
    pub description: String
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
    pub final_score: Option<f32>
}