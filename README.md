# Required Tools

- cargo-make
- musl-gcc
- Login to any Docker Registry

# Deploy

1. `cargo make build_and_push_operator`
2. `cargo make install`
3. `kubectl apply -f config/samples/cr.yaml -n tekito-operator`