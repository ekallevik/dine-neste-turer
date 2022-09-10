use rusqlite::{Connection};
use paris::info;
use crate::repository::insert_activity;

mod dnt_scraper;
mod domain;
mod notifier;
mod repository;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("src/migrations");
}

fn main() {

    info!("Dine neste turer venter! 🥾🎿🧗‍\n️");

    info!("Setting up DB connection");
    let mut conn = Connection::open("dine-neste-turer.sqlite").unwrap();
    embedded::migrations::runner().run(&mut conn).unwrap();

    let source = "https://www.dnt.no/aktiviteter/?audiences=adults%2Cyouth%2Cmountaineers&difficulties=hard%2Cdemanding&organizers=forening%3A2%2Cforening%3A23";
    info!("Looking for activities at: \n\t{}\n", source);
    let activities = dnt_scraper::scrap_activities(source);

    let new_activities: Vec<_> = activities
        .into_iter()
        .filter(|a| !repository::activity_exist(&conn, &a.source))
        .collect();

    for activity in &new_activities {
        insert_activity(&conn, activity);
    }


    notifier::notify_user(&new_activities);
}
