#!/bin/bash
aws lambda create-function --function-name rustTest \
    --handler bootstrap \
    --zip-file fileb://./target/lambda/basic/bootstrap.zip \
    --runtime provided.al2 \
    --role arn:aws:iam::XXXXXXXXXXXXX:role/your_lambda_execution_role \
    --environment Variables={RUST_BACKTRACE=1} \
    --tracing-config Mode=Active