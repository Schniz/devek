#!/bin/bash

cargo run --quiet -- --kdl-usage >docs/usage.kdl
usage generate md --file docs/usage.kdl --out-file README.md
