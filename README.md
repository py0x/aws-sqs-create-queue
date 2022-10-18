# aws-sqs-create-queue
Create multiple AWS SQS queues and its corresponding dead-letter-queues with minimal configuration.

```
$ aws-sqs-create-queue --help

aws-sqs-create-queue 0.1.0
PY <py@pypy.info>
Create multiple AWS SQS queues and its corresponding dead-letter-queues with minimal configuration.

USAGE:
    aws-sqs-create-queue --config <CONFIG>

OPTIONS:
    -c, --config <CONFIG>    The input file containing the sqs configurations
    -h, --help               Print help information
    -V, --version            Print version information
```

## Configuration file
`examples/sqs-conf.toml`
```toml
# --- demo-queue
[[queues]]
[queues.queue]
name = "demo-queue"
visibility_timeout=3500
message_retention_period=3600
message_max_receive_count=3

[queues.dead_letter_queue]
name = "demo-queue-dlq"
visibility_timeout=80
message_retention_period=60


# --- demo-queue-2
[[queues]]
[queues.queue]
name = "demo-queue-2"
visibility_timeout=1234
message_retention_period=111
message_max_receive_count=12

[queues.dead_letter_queue]
name = "demo-queue-2-dlq"
visibility_timeout=180
message_retention_period=601

# --- more
```

## Usage
```
$ aws-sqs-create-queue --config examples/sqs-conf.toml

Ok:
        queue: https://sqs.cn-northwest-1.amazonaws.com.cn/<your_aws_account_id>/demo-queue,
        dead_letter_queue: Some("https://sqs.cn-northwest-1.amazonaws.com.cn/<your_aws_account_id>/demo-queue-dlq")
Ok:
        queue: https://sqs.cn-northwest-1.amazonaws.com.cn/<your_aws_account_id>/demo-queue-2,
        dead_letter_queue: Some("https://sqs.cn-northwest-1.amazonaws.com.cn/<your_aws_account_id>/demo-queue-2-dlq")

```