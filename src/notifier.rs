use paris::info;
use crate::Activity;

pub fn notify_user(activities: Vec<Activity>) {

    for activity in activities {
        info!("New activity{}", activity)
    }
}
