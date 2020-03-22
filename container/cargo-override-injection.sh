#!/usr/bin/env bash

# little hack for overwrites
# since we can not enable features in workspaces
# we launch the sub projects BUT we need to inject
# the overwrites there
echo >> Cargo.toml << EOF

[patch.crates-io]
coaster-nn = { path = "../coaster-nn" }
coaster-blas = { path = "../coaster-blas" }
coaster = { path = "../coaster" }
greenglas = { path = "../greenglas" }
juice = { path = "../juice" }
rust-blas = { path = "../rust-blas" }
rcublas = { path = "../rcublas/cublas" }
rcublas-sys = { path = "../rcublas/cublas-sys" }
rcudnn = { path = "../rcudnn/cudnn" }
rcudnn-sys = { path = "../rcudnn/cudnn-sys" }
EOF