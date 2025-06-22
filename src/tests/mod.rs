use crate::debug_println;

#[macro_export]
macro_rules! setup_test_environment {
    () => {{
        let dir = tempfile::tempdir().unwrap();
        std::env::set_var("HOME", dir.path());
        std::env::set_var("DATABASE_URL", dir.path().join("data.db"));
        let mut cmd = Command::cargo_bin("student_datahub").unwrap();
        cmd.arg("setup").assert().success();
        let db_url = dir.path().join("data.db");
        (dir, SqliteConnection::establish(db_url.to_str().unwrap()).unwrap())
    }};
}