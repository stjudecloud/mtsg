# St. Jude Mutational Spectrum (dev)

The main script runs multiple containers with a pre-built Mutational Spectrum
image. See `README.md` for a description of the process.

## Build

```
$ docker build --tag mutspec ../..
$ dx-docker add-to-applet mutspec .
$ dx-build
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
