# St. Jude Mutational Signatures (dev)

The main script runs multiple containers with a pre-built Mutational Signatures
image. See `README.md` for a description of the process.

## Build

```
$ docker build --tag mtsg ../..
$ mkdir resources/tmp
$ docker save mtsg | gzip > resources/tmp/mtsg-latest.tar.gz
$ dx build
```

## Test

The included tests are a suite of system tests that run the main entry point
with various input arguments. The suite is run locally, preparing input data
manually and shimming dxpy commands.

Tests can be run using [shunit2].

```
$ test/run
```

[shunit2]: https://github.com/kward/shunit2

## Versioning

The version set in `dxapp.json` is defined as "{upstream version}-{release
count}". The upstream version is the same as the application being wrapped and
must not include hyphens. The release count starts at 1 and indicates changes
to the DNAnexus app build.

## Publish

Create and publish an app to make it available in the global namespace to
authorized users.

```
dx build --app --publish
```
