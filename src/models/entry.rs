use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::{RunQueryDsl, SqliteConnection};
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use crate::models::{Entry, Period};
use crate::schema::entry::{date, subject_id};
use crate::schema::entry::dsl::entry;
use crate::schema::periods::dsl::periods;
use crate::schema::periods::{final_date, initial_date};

impl Entry {
    /// Gets the period to which the entry belongs. If it doesn't belong to any period (it should), returns `None`.
    /// # Arguments
    /// * conn - Database connection
    pub fn get_period(&self, conn: &mut SqliteConnection) -> Option<Period> {
        periods
            .filter(initial_date.le(&self.date))
            .filter(final_date.ge(&self.date))
            .load::<Period>(conn)
            .expect("Error loading period")
            .pop()
    }

    /// Fetches a vector with all entries from a single date.
    /// # Arguments
    /// * `date_to_fetch` - Date from which we want the entries
    /// * `conn` - Database connection
    pub fn fetch_by_day(date_to_fetch: NaiveDate, conn: &mut SqliteConnection) -> Vec<Entry> {
        entry
            .filter(date.eq(&date_to_fetch))
            .load::<Entry>(conn)
            .expect("Error loading entry")
    }
    
    /// Fetches a vector with all entries in a given date interval.
    /// # Arguments
    /// * `conn`- Database connection
    /// * `interval` - Date interval (start, end)
    pub fn fetch_by_interval(conn : &mut SqliteConnection, interval: (Option<NaiveDate>, Option<NaiveDate>)) -> Vec<Entry> {
        match interval {
            (Some(s), Some(e)) => {
                entry
                    .filter(date.ge(&s))
                    .filter(date.le(&e))
                    .load::<Entry>(conn)
                    .expect("Error loading entry")
            }
            (Some(s), None) => {
                entry
                    .filter(date.ge(&s))
                    .load::<Entry>(conn)
                    .expect("Error loading entry")
            }
            (None, Some(e)) => {
                entry
                    .filter(date.le(&e))
                    .load::<Entry>(conn)
                    .expect("Error loading entry")
            }
            (None, None) => {
                entry
                    .load::<Entry>(conn)
                    .expect("Error loading entry")
            }
        }
    }

    /// Gets the dedicated time to a subject in a determined day. If there was no entry regarding that date, returns zero.
    /// # Arguments
    /// * `date_to_fetch` - date to search.
    /// * `subject_to_fetch` - subject id to search.
    /// * `conn` - connection to the database.
    pub fn get_time_by_day_and_subject(
        date_to_fetch: NaiveDate,
        subject_to_fetch: i32,
        conn: &mut SqliteConnection,
    ) -> i32 {
        match entry
            .filter(date.eq(&date_to_fetch))
            .filter(subject_id.eq(&subject_to_fetch))
            .load::<Entry>(conn)
            .expect("Failed to fetch entries")
            .first()
        {
            Some(e) => e.dedicated_time,
            None => 0,
        }
    }
}