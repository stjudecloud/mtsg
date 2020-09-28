# St. Jude Mutational Signatures (dev)

The main script runs multiple containers with a pre-built Mutational Signatures
image.

## Build

```
$ docker build --tag mtsg ../..
$ mkdir resources/tmp
$ docker save mtsg | zstd -T0 -f -o resources/tmp/mtsg-latest.tar.zst
$ dx build
```

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
