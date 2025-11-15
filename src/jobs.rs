use crate::controllers::{Controller, competitions::COMPS, live::LIVE};
use tokio_cron_scheduler::{Job, JobSchedulerError};

pub fn jobs() -> Result<impl Iterator<Item = Job>, JobSchedulerError> {
    // Format: sec, min, hour, day of month, month, day of week
    Ok([
        Job::new_async("0 0 6 * * *", |_, _| {
            Box::pin(async move {
                if let Some(comps) = COMPS.get() {
                    // TODO: warn!
                    let _ = comps.lock().await.update_and_send().await;
                }
            })
        })?,
        Job::new_async("0 0 * * * *", |_, _| {
            Box::pin(async move {
                if let Some(live) = LIVE.get() {
                    // TODO: warn!
                    let _ = live.lock().await.update_and_send().await;
                }
            })
        })?,
    ]
    .into_iter())
}
