[![Crates.io](https://img.shields.io/crates/v/pac-man.svg)](https://crates.io/crates/pac-man)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/gregl83/pac-man/blob/master/LICENSE)
[![Build Status](https://github.com/gregl83/pac-man/workflows/CI/badge.svg?branch=main)](https://github.com/gregl83/pac-man/actions?query=workflow%3ACI+branch%3Amain)
# pac-man

AWS Lambda streaming API consumer

## Stability

Experimental

## Service Dependencies

**Required**

- AWS Lambda
- AWS S3

**Optional**

- GitHub Actions CI/CD
- AWS Secrets Manager

## Testing

Lambda functions can be executed with the help of [Docker](https://github.com/awslabs/aws-lambda-rust-runtime#docker).

For convenience, [docker-test.sh](/docker-test.sh) launches a lambda build using docker (requires `~/.aws/credentials`).

### Lambda Build & Run

**Build Lambda Package**
```bash
../pac-man$ docker run --rm \
      -v ${PWD}:/code \
      -v ${HOME}/.cargo/registry:/root/.cargo/registry \
      -v ${HOME}/.cargo/git:/root/.cargo/git \
      softprops/lambda-rust
```

**Unzip Lambda Package**
```bash
../pac-man$ unzip -o \
                target/lambda/release/pac-man.zip \
                -d /tmp/lambda
```

**Run Unzipped Lambda Package**
```bash
../pac-man$ docker run \
                -i -e DOCKER_LAMBDA_USE_STDIN=1 \
                --rm \
                -v /tmp/lambda:/var/task \
                lambci/lambda:provided
```

## Todos

- Handle paginated APIs

## License

[MIT](LICENSE)
