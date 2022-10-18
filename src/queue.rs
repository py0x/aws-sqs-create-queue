use aws_sdk_sqs as sqs;
use sqs::model::QueueAttributeName;
use anyhow::{Context, Result};
use serde_json;
use serde_json::json;

pub struct CreateQueueArgs {
    pub queue_name: String,
    pub visibility_timeout: u32,
    pub message_retention_period: u32,
    pub dead_letter_queque_arn: Option<String>,
    pub message_max_receive_count: Option<u32>,  // default to 3
}

pub async fn create_queue(client: &sqs::Client, args: &CreateQueueArgs) -> Result<String> {
    let queue_name = &args.queue_name;

    let mut req = client.create_queue()
        .queue_name(queue_name)
        .attributes(
            QueueAttributeName::VisibilityTimeout,
            args.visibility_timeout.to_string(),
        )
        .attributes(
            QueueAttributeName::MessageRetentionPeriod,
            args.message_retention_period.to_string(),
        );

    // set dlq
    if let Some(dead_letter_queque_arn) = &args.dead_letter_queque_arn {
        let message_max_receive_count = &args.message_max_receive_count.unwrap_or(3);
        req = req.attributes(
            QueueAttributeName::RedrivePolicy,
            (json!({
                    "deadLetterTargetArn": dead_letter_queque_arn,
                    "maxReceiveCount": message_max_receive_count,
            })).to_string(),
        )
    }

    let res = req.send().await
        .context(format!("Failed to create queue {queue_name}"))?;

    let queue_url = res.queue_url()
        .context(format!("Failed to extract queue_url for queue {queue_name}"))?;

    Ok(queue_url.to_string())
}

pub async fn get_queue_arn(client: &sqs::Client, queue_url: &str) -> Result<String> {
    let queue_attr_res = client.get_queue_attributes()
        .queue_url(queue_url)
        .attribute_names(QueueAttributeName::QueueArn)
        .send()
        .await
        .context(format!("Failed to get_queue_attributes for queue {queue_url}"))?;


    let queue_attr_map = queue_attr_res.attributes()
        .context(format!("Failed to get queue attributes for queue {queue_url}"))?;


    let queue_arn = queue_attr_map.get(&QueueAttributeName::QueueArn)
        .context(format!("Failed to extract queue attributes for queue {queue_url}"))?;


    Ok(queue_arn.to_string())
}