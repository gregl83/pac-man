#!/usr/bin/env bash

# get run options
while test $# -gt 0; do
  case "$1" in
    -h|--help)
      echo "pac-man$ docker-test - run lambda package"
      echo " "
      echo "pac-man$ docker-test [options]"
      echo " "
      echo "options:"
      echo "-h, --help                show brief help"
      echo "-b, --build               build lambda package prior to running"
      exit 0
      ;;
    -b|--build)
      shift
      export PACMAN_BUILD=1
      ;;
    *)
      break
      ;;
  esac
done

# cd to pac-man directory
cd "$(dirname "$0")"

if [[ -n ${PACMAN_BUILD} && "${PACMAN_BUILD}"=="1" ]]; then
  # build lambda package
  docker run --rm \
      -v ${PWD}:/code \
      -v ${HOME}/.cargo/registry:/root/.cargo/registry \
      -v ${HOME}/.cargo/git:/root/.cargo/git \
      softprops/lambda-rust && \
  unzip -o \
      target/lambda/release/pac-man.zip \
      -d /tmp/lambda && \
  echo "Enter Payload Then Press CTRL-D..." && \
  docker run \
      -i -e DOCKER_LAMBDA_USE_STDIN=1 \
      -e AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID} \
      -e AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY} \
      --rm \
      -v /tmp/lambda:/var/task \
      lambci/lambda:provided
else
  echo "Enter Payload Then Press CTRL-D..." && \
  docker run \
      -i -e DOCKER_LAMBDA_USE_STDIN=1 \
      -e AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID} \
      -e AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY} \
      --rm \
      -v /tmp/lambda:/var/task \
      lambci/lambda:provided
fi
