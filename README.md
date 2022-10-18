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
Please refer to `examples/sqs-conf.toml`

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