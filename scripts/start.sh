#!/bin/bash

BIND_DIR="./generated"

if [ ! -d "$BIND_DIR" ]; then
  echo "Creating export directory: $BIND_DIR"
  mkdir -p "$BIND_DIR"
fi

make docker-up