use crate::domain::activity::Activity;
use paris::info;
use rusqlite::{params, Connection};

pub fn insert_activity(conn: &Connection, activity: &Activity) {
    if activity_exist(conn, &activity.source) {
        info!(
            "Activity {} already exists, skipping insertion",
            &activity.source
        )
    } else {
        info!("Inserting activity {}", &activity.title);

        let params = params![
            activity.title,
            activity.category,
            activity.date.unwrap_or_default().to_string(),
            activity.duration.map(|v| v.num_hours()),
            activity.description,
            activity.get_audiences_as_string(),
            activity.organizer,
            activity.source
        ];

        // language=sqlite
        let sql = "
        INSERT INTO activity (title, category, date, duration_in_hours, description, audiences, organizer, source)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
    ";

        conn.execute(sql, params).expect("TODO: panic message");
    }
}

pub fn activity_exist(conn: &Connection, source: &String) -> bool {
    info!("Checking if source exists: {source}");

    let params = [source];

    // language=sqlite
    let sql = "SELECT title FROM activity where source = ?1";
    let mut statement = conn.prepare(sql).unwrap();

    let activity = statement.query_map(params, |_| Ok(true)).unwrap();

    activity.count() > 0
}
