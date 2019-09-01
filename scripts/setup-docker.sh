#!/usr/bin/env bash

apt-get update;
apt-get install -y gnupg2;
apt-get install -y apt-transport-https ca-certificates curl software-properties-common;
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -;
add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu bionic test";
apt-get -y update && apt-get -y upgrade;
apt install -y docker-ce docker-compose;
docker --version;