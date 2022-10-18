use aws_sdk_sqs as sqs;
use anyhow::{Context, Result};
use std::option::Option;
use clap::Parser;

mod config;
mod queue;
mod cli;

use queue::{
    create_queue,
    get_queue_arn,
    CreateQueueArgs,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    let config = aws_config::load_from_env().await;
    let client = sqs::Client::new(&config);

    let queue_configs = config::load_queue_config(&args.config)?;

    for queue_conf in queue_configs.iter() {
        let mut dlq_url: Option<String> = None;
        let mut dlq_arn: Option<String> = None;

        if let Some(dlq_attr) = &queue_conf.dead_letter_queue {
            let dlq_queue_url = create_queue(&client, &CreateQueueArgs {
                queue_name: dlq_attr.name.to_string(),
                visibility_timeout: dlq_attr.visibility_timeout.to_owned(),
                message_retention_period: dlq_attr.message_retention_period.to_owned(),
                dead_letter_queque_arn: None,
                message_max_receive_count: None,
            })
                .await
                .context(format!("Failed to create dead letter queue: {}", dlq_attr.name))?;

            let dlq_queue_arn = get_queue_arn(&client, &dlq_queue_url)
                .await
                .context(format!("Failed to get arn for dead letter queue: {}", dlq_attr.name))?;

            dlq_url = Some(dlq_queue_url);
            dlq_arn = Some(dlq_queue_arn);
        }

        let queue_attr = &queue_conf.queue;
        let queue_url = create_queue(&client, &CreateQueueArgs {
            queue_name: queue_attr.name.to_string(),
            visibility_timeout: queue_attr.visibility_timeout.to_owned(),
            message_retention_period: queue_attr.message_retention_period.to_owned(),
            dead_letter_queque_arn: dlq_arn.to_owned(),
            message_max_receive_count: queue_attr.message_max_receive_count.to_owned(),
        })
            .await
            .context(format!("Failed to create queue: {}", queue_attr.name))?;

        println!("Ok:\n\tqueue: {},\n\tdead_letter_queue: {:?}", queue_url, dlq_url);
    }

    Ok(())
}
