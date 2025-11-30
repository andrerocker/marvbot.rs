use tokio::sync::mpsc::{self, Sender};
use tokio_cron_scheduler::{Job, JobScheduler};

pub fn spawn(sender: Sender<Vec<String>>) -> tokio::task::JoinHandle<()> {
    tokio::task::spawn(async move {
        let scheduled = execute(async move |response: Vec<String>| {
            sender.send(response).await.unwrap();
        })
        .await;

        if let Err(error) = scheduled {
            log::error!("Problems processeing scheduled tasks: {}", error);
            // should we interrupt the program execution? just log for now.
        }
    })
}

pub async fn execute<F: AsyncFnMut(Vec<String>)>(mut callback: F) -> anyhow::Result<()> {
    let scheduler = JobScheduler::new().await?;
    let schedulables = crate::marv::plugins::default_schedulables();
    let (writer, mut receiver) = mpsc::channel::<Vec<String>>(10);

    for (appointment, plugin) in schedulables {
        let writer = writer.clone();
        let job = Job::new_async(appointment, move |_uuid, _l| {
            let writer = writer.clone();
            Box::pin(async move {
                let response = plugin.perform(&"schedule".to_string()).await.unwrap();
                writer.send(response).await.unwrap();
                ()
            })
        })?;

        scheduler.add(job).await?;
    }

    scheduler.start().await?;

    while let Some(msg) = receiver.recv().await {
        callback(msg).await;
    }

    Ok(())
}
