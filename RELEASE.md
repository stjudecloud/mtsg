# Release

  * [ ] Update `version` in `Cargo.toml`.
  * [ ] Update `version` and `upstreamVersion` in `dnanexus/mtsg/dxapp.json`.
  * [ ] Update `CHANGELOG.md` with version and publication date.
  * [ ] Run tests: `cargo test`. (This will also update `Cargo.lock`.)
  * [ ] Run DNAnexus tests: `dnanexus/mtsg/test/run`
  * [ ] Stage changes: `git add Cargo.lock Cargo.toml CHANGELOG.md dnanexus/mtsg/dxapp.json`
  * [ ] Commit changes: `git commit -m "Bump version to $VERSION"`
  * [ ] Create git tag: `git tag -m "" -a v$VERSION`
  * [ ] Push release: `git push --follow-tags`

## DNAnexus

  * [ ] Build container image: `docker image build --tag mtsg .`
  * [ ] Save container image: `docker image save mtsg | gzip > dnanexus/mtsg/resources/tmp/mtsg-latest.tar.gz`
  * [ ] Check security context: `dx whoami`
  * [ ] Build DNAnexus applet: `dx build --destination mtsg:/mtsg-$VERSION dnanexus/mtsg`
  * [ ] Publish DNAnexus app: `dx build --app --publish dnanexus/mtsg`
  * [ ] Build St. Jude Cloud production workflow.
