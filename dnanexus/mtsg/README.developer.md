# St. Jude Mutational Signatures (dev)

The main script runs multiple containers with a pre-built Mutational Signatures
image. See `README.md` for a description of the process.

## Build

```
$ docker build --tag mtsg ../..
$ dx-docker add-to-applet mtsg .
$ dx build
```

Note dx-docker exports the image in the ACI format, which requires
[docker2aci] to be installed.

[docker2aci]: https://github.com/appc/docker2aci

## Test

The included tests are a suite of system tests that run the main entry point
with various input arguments. The suite is run locally, preparing input data
manually and shimming dxpy commands.

Tests can be run using [shunit2].

```
$ test/run
```

[shunit2]: https://github.com/kward/shunit2

## Publish

Create and publish an app to make it available in the global namespace to
authorized users.

```
dx build --app --publish
```
