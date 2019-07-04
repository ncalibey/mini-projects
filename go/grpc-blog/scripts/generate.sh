#!/bin/bash

protoc -I ../blogpb ../blogpb/blog.proto --go_out=plugins=grpc:../blogpb
