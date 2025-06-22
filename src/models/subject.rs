use std::fmt::{Display, Formatter};
use crate::models::{Entry, Subject};
use crate::schema::entry::dsl::entry;
use crate::schema::entry::{date, dedicated_time, subject_id};
use crate::schema::subjects::dsl::subjects;
use diesel::dsl::sum;
use diesel::internal::derives::multiconnection::chrono::{NaiveDate, NaiveWeek};
use diesel::QueryDsl;
use diesel::SqliteConnection;
use diesel::{ExpressionMethods, RunQueryDsl};
use std::process;

impl Display for Subject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.final_score.is_some() {
            write!(f, "{} ({}) [{}]",
                   self.name,
                   self.short_name,
                   self.final_score.unwrap())
        }
        else {
            write!(f, "{} ({})", self.name, self.short_name)
        }

    }
}

impl Subject {
    /// Gets the total dedicated time of the subject
    /// # Arguments
    /// * `conn` - Database connection
    pub fn total_dedicated_time(&self, conn: &mut SqliteConnection) -> i32 {
        entry
            .select(sum(dedicated_time))
            .filter(subject_id.eq(self.id))
            .first::<Option<i64>>(conn)
            .expect("Error loading entry")
            .unwrap_or(0) as i32
    }
    /// Gets the total dedicated time of the subject in an interval.
    /// # Arguments
    /// * `conn` - Database connection
    /// * `interval` - Interval, where [None] means infinite
    pub fn total_dedicated_time_interval(
        &self,
        conn: &mut SqliteConnection,
        interval: (NaiveDate, NaiveDate),
    ) -> i32 {
        entry
            .select(sum(dedicated_time))
            .filter(subject_id.eq(self.id))
            .filter(date.ge(interval.0))
            .filter(date.le(interval.1))
            .first::<Option<i64>>(conn)
            .expect("Error loading entry")
            .unwrap_or(0) as i32
    }
    pub fn total_dedicated_time_week(&self, conn: &mut SqliteConnection, week: NaiveWeek) -> i32 {
        self.total_dedicated_time_interval(conn, (week.first_day(), week.last_day()))
    }
    /// Gets the dedicated time to a subject in a determined day. If there was no entry regarding that date, returns zero.
    /// # Arguments
    /// * `date_to_fetch` - date to search.
    /// * `subject_to_fetch` - subject id to search.
    /// * `conn` - connection to the database.
    pub fn total_dedicated_time_day(
        &self,
        date_to_fetch: NaiveDate,
        conn: &mut SqliteConnection,
    ) -> i32 {
        match entry
            .filter(date.eq(&date_to_fetch))
            .filter(subject_id.eq(self.id))
            .load::<Entry>(conn)
            .expect("Failed to fetch entries")
            .first()
        {
            Some(e) => e.dedicated_time,
            None => 0,
        }
    }

    /// Fetches all entries related to the subject.
    /// # Arguments
    /// * `conn` - Database connection
    pub fn fetch_entries(&self, conn: &mut SqliteConnection) -> Vec<Entry> {
        match entry.filter(subject_id.eq(self.id)).load::<Entry>(conn) {
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
