//! Module with diesel's database models.
#![allow(dead_code)]
use crate::schema::entry::dsl::entry;
use crate::schema::entry::{date, subject_id};
use crate::schema::periods::dsl::periods;
use crate::schema::periods::{final_date, initial_date};
use crate::{debug_println, FORMAT};
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate};
use diesel::prelude::*;
use std::process;
use crate::schema::subjects::dsl::subjects;

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

impl Period {
    /// Gets a formatted string with relevant data of the period.
    pub fn to_string(&self) -> String {
        format!(
            "{} - {}\t{} (ID:{})",
            self.initial_date.format(FORMAT).to_string(),
            self.final_date.format(FORMAT).to_string(),
            self.description.to_string(),
            self.id
        )
    }
    /// It determines if the period is actual (It is ocurring now)
    pub fn is_actual(&self) -> bool {
        let now = Local::now().date_naive();
        if now >= self.initial_date && now <= self.final_date {
            true
        } else {
            false
        }
    }
    /// It fetches all study periods.
    /// # Arguments
    /// * `conn` - Database connection
    pub fn fetch_all_plans(conn: &mut SqliteConnection) -> Vec<Period> {
        match periods.load::<Period>(conn) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Failed to load the periods.");
                debug_println!("{e}");
                process::exit(1);
            }
        }
    }

    /// Gets the period given a date. Returns None if there isn't any.
    /// # Arguments
    /// * `conn` - Database connection.
    /// * `period_date` - Date of the period.
    pub fn from_date(conn: &mut SqliteConnection, period_date: &NaiveDate) -> Option<Period> {
        match periods
            .filter(initial_date.le(period_date))
            .filter(final_date.ge(period_date))
            .load::<Period>(conn)
        {
            Ok(period) => {
                if period.len() > 1 {
                    debug_println!(
                        "There is more than one period ocurring now! Content: {:?}",
                        period
                    );
                }
                period.first().cloned()
            }
            Err(e) => {
                eprintln!("Failed to load: {e}");
                process::exit(1);
            }
        }
    }

    /// Gets the period given its id. Returns None if there isn't any.
    /// # Arguments
    /// * `conn` - Database connection
    /// * `id_to_fetch` - Period id.
    pub fn from_id(conn: &mut SqliteConnection, id_to_fetch: i32) -> Option<Period> {
        match periods
            .filter(crate::schema::periods::id.eq(id_to_fetch))
            .load::<Period>(conn)
        {
            Ok(p) => p.first().cloned(),
            Err(e) => {
                eprintln!("Failed to fetch period: {e}");
                process::exit(1);
            }
        }
    }

    /// Gets the period ocurring now.
    /// # Arguments
    /// * `conn` - Database connection
    pub fn get_actual_period(conn: &mut SqliteConnection) -> Option<Period> {
        Self::from_date(conn, &Local::now().date_naive())
    }

    /// It determines whether the period is overlaping another.
    pub fn overlaps_period(&self, other: &Period) -> bool {
        let p2 = (other.initial_date, other.final_date);
        self.overlaps(p2)
    }
    /// It determines whether the period is overlaping the period between `dates`.
    /// # Arguments
    /// * `dates` - Date interval (start, end)
    pub fn overlaps(&self, dates: (NaiveDate, NaiveDate)) -> bool {
        let p1 = (self.initial_date, self.final_date);
        (p1.0 <= dates.1 && p1.0 >= dates.0)
            || (p1.1 <= dates.1 && p1.1 >= dates.0)
            || (p1.0 <= dates.0 && p1.1 >= dates.0)
    }
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
    /// Fetches all subjects from the database.
    /// # Arguments:
    /// * conn - Database connection.
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
