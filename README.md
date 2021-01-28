[![Crates.io](https://img.shields.io/crates/v/pac-man.svg)](https://crates.io/crates/pac-man)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/gregl83/pac-man/blob/master/LICENSE)
[![Build Status](https://github.com/gregl83/pac-man/workflows/CI/badge.svg?branch=main)](https://github.com/gregl83/pac-man/actions?query=workflow%3ACI+branch%3Amain)
# pac-man

AWS Lambda streaming API consumer

## Stability

Experimental

## Service Dependencies

**Required**

- [AWS Lambda](https://aws.amazon.com/lambda/)
- [AWS S3](https://aws.amazon.com/s3/)

**Optional**

- [GitHub Actions CI/CD](https://github.com/features/actions)
- [AWS Secrets Manager](https://aws.amazon.com/secrets-managser/)
- [AWS EventBridge](https://aws.amazon.com/eventbridge/)

## Usage

AWS Lambda deployments can be triggered quite a bit of ways. Configuring an event to trigger your Lambda job will involve parameters respective to your API Source and target Destination.

Uncomfortable with sensitive values in plaintext? Good, you should be!

AWS Secrets Manager is supported using the following Lambda Event Value format: `{:secrets:<name>:<key>}`.

**Minima Lambda Event**

```json
{
  "source": {
    "scheme": "https",
    "hostname": "example.com"
  },
  "destination": {
    "region": "us-east-1",
    "collection": "bucket-name",
    "name": "key"
  }
}
```

**Maxima Lambda Event**

```json
{
  "modules": {
      "secrets": {
          "region": "us-east-1"      
      }
  },
  "source": {
    "scheme": "https",
    "username": "pseudo",
    "password": "{:secrets:pac-man:pw}",
    "hostname": "example.com",
    "port": 8080,
    "path": "/follow/the",
    "params": {
      "name": "value"    
    },
    "fragment": "/yellow/brick/road"
  },
  "destination": {
    "region": "us-east-1",
    "collection": "bucket-name",
    "name": "key"
  }
}
```

**Optional Event Fields**

`delta( minima.fields, maxima.fields )`

## Testing

Lambda functions can be executed with the help of [Docker](https://github.com/awslabs/aws-lambda-rust-runtime#docker).

### Docker Containers

[docker-test.sh](/docker-test.sh) launches a Lambda build using Docker.

#### Environment Variables

- AWS_SECRET_ACCESS_KEY
- AWS_ACCESS_KEY_ID

#### 1. Build Package

```bash
../pac-man$ docker run --rm \
      -v ${PWD}:/code \
      -v ${HOME}/.cargo/registry:/root/.cargo/registry \
      -v ${HOME}/.cargo/git:/root/.cargo/git \
      softprops/lambda-rust
```

#### 2. Unzip Package

```bash
../pac-man$ unzip -o \
      target/lambda/release/pac-man.zip \
      -d /tmp/lambda
```

#### 3. Run Unzipped Package
```bash
../pac-man$ docker run \
      -i -e DOCKER_LAMBDA_USE_STDIN=1 \
      -e AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID} \
      -e AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY} \
      --rm \
      -v /tmp/lambda:/var/task \
      lambci/lambda:provided
```

## References

- [W3 URI Specification](https://www.w3.org/Addressing/URL/uri-spec.html)

## License

[MIT](LICENSE)
