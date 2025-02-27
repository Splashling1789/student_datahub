use std::process;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use diesel::QueryDsl;
use diesel::SqliteConnection;
use crate::models::{Entry, Subject};
use crate::schema::entry::dsl::entry;
use crate::schema::entry::{date, subject_id};
use crate::schema::subjects::dsl::subjects;

impl Subject {
    /// Gets a formatted string with relevant data of the subject.
    pub fn to_string(&self) -> String {
        if self.final_score.is_some() {
            format!(
                "{} ({}) [{}]",
                self.name,
                self.short_name,
                self.final_score.unwrap()
            )
        } else {
            format!("{} ({})", self.name, self.short_name)
        }
    }
    /// Gets the total dedicated time of the subject
    /// # Arguments
    /// * `conn` - Database connection
    pub fn total_dedicated_time(&self, conn: &mut SqliteConnection) -> i32 {
        entry
            .filter(subject_id.eq(self.id))
            .load::<Entry>(conn)
            .expect("Error loading entry")
            .iter()
            .map(|e| e.dedicated_time)
            .sum()
    }
    /// Gets the total dedicated time of the subject in an interval.
    /// # Arguments
    /// * `conn` - Database connection
    /// * `interval` - Interval, where [None] means infinite
    pub fn total_dedicated_time_interval(
        &self,
        conn: &mut SqliteConnection,
        interval: (Option<NaiveDate>, Option<NaiveDate>),
    ) -> i32 {
        match interval {
            (None, None) => self.total_dedicated_time(conn),
            (Some(k), None) => entry
                .filter(subject_id.eq(self.id))
                .filter(date.ge(k))
                .load::<Entry>(conn)
                .expect("Error loading entry")
                .iter()
                .map(|e| e.dedicated_time)
                .sum(),
            (None, Some(k)) => entry
                .filter(subject_id.eq(self.id))
                .filter(date.le(k))
                .load::<Entry>(conn)
                .expect("Error loading entry")
                .iter()
                .map(|e| e.dedicated_time)
                .sum(),
            (Some(k), Some(j)) => entry
                .filter(subject_id.eq(self.id))
                .filter(date.ge(k))
                .filter(date.le(j))
                .load::<Entry>(conn)
                .expect("Error loading entry")
                .iter()
                .map(|e| e.dedicated_time)
                .sum(),
        }
    }
    
    /// Fetches all entries related to the subject.
    /// # Arguments
    /// * `conn` - Database connection
    pub fn fetch_entries(&self, conn: &mut SqliteConnection) -> Vec<Entry> {
        match entry
            .filter(subject_id.eq(self.id))
            .load::<Entry>(conn) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to fetch entry: {}", e);
                process::exit(1);
            }
        }
    }
    
    /// Fetches all subjects from the database.
    /// # Arguments:
    /// * `conn` - Database connection.
    pub fn fetch_all(conn: &mut SqliteConnection) -> Vec<Subject> {
        match subjects.load::<Subject>(conn) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Failed to fetch subjects: {}", e);
                process::exit(1);
            }
        }
    }
}