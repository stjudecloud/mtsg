# St. Jude Mutational Spectrum (dev)

The main script runs a container with a pre-build Mutational Spectrum image.
The DNAnexus applet executes the following workflow:

  * splits a multi-sample VCF to multiple single-sample VCFs
  * generates a sample sheet from the directory of single-sample VCFs
  * runs mutational patterns
  * creates a visualization file using the fitted signatures

## Build

```
$ docker build --tag mutspec ../..
$ dx-docker add-to-applet mutspec .
$ dx-build
```

Note dx-docker exports the image in the ACI format, which requires
[docker2aci] to be installed.

[docker2aci]: https://github.com/appc/docker2aci
