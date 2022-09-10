use crate::domain::activity::Activity;
use paris::info;

pub fn notify_user(activities: &Vec<Activity>) {
    for activity in activities {
        info!("New activity: {}", activity)
    }
}
