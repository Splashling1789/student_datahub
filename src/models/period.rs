use std::fmt::{Display, Formatter};
use crate::models::{Period, Subject};
use crate::schema::entry::date;
use crate::schema::entry::dsl::entry;
use crate::schema::periods::dsl::periods;
use crate::schema::periods::{final_date, initial_date};
use crate::schema::subjects::dsl::subjects;
use crate::schema::subjects::period_id;
use crate::{debug_println, FORMAT};
use diesel::dsl::sql;
use diesel::internal::derives::multiconnection::chrono::{Local, NaiveDate};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::{RunQueryDsl, SqliteConnection};
use std::process;

impl Display for Period {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}\t{} (ID:{})",
               self.initial_date.format(FORMAT),
               self.final_date.format(FORMAT),
               self.description,
               self.id)
    }
}

impl Period {
    /// It determines if the period is actual (It is ocurring now)
    pub fn is_actual(&self) -> bool {
        let now = Local::now().date_naive();

        now >= self.initial_date && now <= self.final_date
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

    /// Fetches the subjects that belongs to this period.
    /// #Arguments
    /// * `conn` - Database connection
    pub fn fetch_subjects(&self, conn: &mut SqliteConnection) -> Vec<Subject> {
        match subjects.filter(period_id.eq(self.id)).load::<Subject>(conn) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to fetch the subjects: {e}");
                process::exit(1);
            }
        }
    }

    /// Gets the global dedicated time average by weeks until de given day.
    /// # Arguments
    /// * `conn` - Database connection.
    /// * `until` - Day which previous week is the last to be calculated.
    /// * `period` - Period to fetch times.
    pub fn weekly_average_until(
        &self,
        conn: &mut SqliteConnection,
        from: NaiveDate,
        until: NaiveDate,
    ) -> f64 {
        let start = from.max(self.initial_date);
        let end = until.min(self.final_date);
        let weekly_sum = entry
            .select((
                sql::<diesel::sql_types::Integer>("strftime('%Y', date)"),
                sql::<diesel::sql_types::Integer>("strftime('%W', date)"),
                sql::<diesel::sql_types::Nullable<diesel::sql_types::Integer>>(
                    "SUM(dedicated_time) AS suma",
                ),
            ))
            .filter(date.between(start, end))
            .group_by((
                sql::<diesel::sql_types::Integer>("strftime('%Y', date)"),
                sql::<diesel::sql_types::Integer>("strftime('%W', date)"),
            ))
            .load::<(i32, i32, Option<i32>)>(conn)
            .expect("Failed to fetch the weekly average");
        weekly_sum.iter().map(|t| t.2.unwrap_or(0)).sum::<i32>() as f64 / (weekly_sum.len() as f64)
    }
}
