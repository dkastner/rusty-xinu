#!/bin/bash 
set -x

docker build -t xinu .

docker create -t --name xinu-build xinu make -d all iso
docker start -ai xinu-build
docker cp xinu-build:/build/build/os.iso .
docker rm -f xinu-build
