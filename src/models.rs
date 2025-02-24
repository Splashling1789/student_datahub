//! Module with diesel's database models.

use std::process;
use crate::schema::periods::dsl::periods;
use crate::schema::periods::{final_date, initial_date};
use crate::{debug_println, FORMAT};
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate};
use diesel::prelude::*;
use crate::schema::entry::date;
use crate::schema::entry::dsl::entry;

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
    pub fn fetch_by_day(date_to_fetch : NaiveDate, conn : &mut SqliteConnection) -> Vec<Entry> {
        entry
            .filter(date.eq(&date_to_fetch))
            .load::<Entry>(conn)
            .expect("Error loading entry")
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
    
    pub fn get_actual_period(conn: &mut SqliteConnection) -> Option<Period> {
        let now = Local::now().date_naive();
        match periods
            .filter(initial_date.le(now))
            .filter(final_date.ge(now))
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
    
    /// It determines whether the period is overlaping another.
    pub fn overlaps_period(&self, other: &Period) -> bool {
        let p2 = (other.initial_date, other.final_date);
        self.overlaps(p2)
    }
    /// It determines whether the period is overlaping the period between `dates`.
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
    pub fn to_string(&self) -> String {
        if self.final_score.is_some() {
            format!("{} ({}) [{}]", self.name, self.short_name, self.final_score.unwrap())
        }
        else {
            format!("{} ({})", self.name, self.short_name)
        }
    }
}
