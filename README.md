# Mutational Signatures

**Mutational Signatures** (abbreviated as **mtsg**) finds and quantifies [COSMIC
mutational signatures] across samples.

[COSMIC mutational signatures]: https://cancer.sanger.ac.uk/cosmic/signatures

## Prerequisites

  * [Python] ^3.8
    * [sigproSS] ^0.0.0

[Python]: https://www.python.org/
[sigproSS]: https://github.com/AlexandrovLab/SigProfilerSingleSample

## Install

Use [Poetry] to install mtsg and its dependencies.

```
$ poetry install --no-dev
```

[Poetry]: http://python-poetry.org/

## Usage

```
usage: main.py [-h] [--dst-prefix DST_PREFIX] [--genome-build {GRCh38}] src-prefix

positional arguments:
  src-prefix

optional arguments:
  -h, --help            show this help message and exit
  --dst-prefix DST_PREFIX
  --genome-build {GRCh38}s
```

## Docker

Mutational Signatures has a `Dockerfile` to create a Docker image, which sets
up and installs the required runtime and dependencies. To build and use this
image, [install Docker](https://docs.docker.com/install) for your platform.

### Build

In the Mutational Signatures project directory, build the Docker image.

```
$ docker image build --tag mtsg .
```

### Run

The image uses `mtsg/main.py` as its entrypoint, giving access to all commands.

```
$ docker container run mtsg <args...>
```
