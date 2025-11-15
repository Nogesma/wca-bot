use crate::controllers::{competitions::COMPS, live::LIVE};
use log::{error, info};
use tokio_cron_scheduler::{Job, JobSchedulerError};

pub fn jobs() -> Result<impl Iterator<Item = Job>, JobSchedulerError> {
    // Format: sec, min, hour, day of month, month, day of week
    Ok([
        Job::new_async("0 0 6 * * *", |_, _| {
            Box::pin(async move {
                info!("Updating wca comps");
                if let Some(comps) = COMPS.get()
                    && let Err(e) = comps.lock().await.update_and_send().await
                {
                    error!("Unable to send/update comps: {e}")
                }
            })
        })?,
        Job::new_async("0 0 * * * *", |_, _| {
            Box::pin(async move {
                info!("Updating wca live");
                if let Some(live) = LIVE.get()
                    && let Err(e) = live.lock().await.update_and_send().await
                {
                    error!("Unable to send/update wcalive: {e}")
                }
            })
        })?,
    ]
    .into_iter())
}
