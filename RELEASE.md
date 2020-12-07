# Release

  * [ ] Update `__version__` in `mtsg/__init__.py`.
  * [ ] Update `version` in `pyproject.toml`.
  * [ ] Update `version` and `upstreamVersion` in `dnanexus/mtsg/dxapp.json`.
  * [ ] Update `CHANGELOG.md` with version and publication date.
  * [ ] Run tests: `poetry run pytest tests`.
  * [ ] Stage changes: `git add mtsg/__init__.py pyproject.toml dnanexus/mtsg/dxapp.json CHANGELOG.md`
  * [ ] Commit changes: `git commit -m "Bump version to $VERSION"`
  * [ ] Create git tag: `git tag -m "" -a v$VERSION`
  * [ ] Push release: `git push --follow-tags`

## DNAnexus

  * [ ] Build container image: `docker image build --tag mtsg .`
  * [ ] Save container image: `docker image save mtsg | zstd -T0 -f -o dnanexus/mtsg/resources/tmp/mtsg-latest.tar.zst`
  * [ ] Check security context: `dx whoami`
  * [ ] Build DNAnexus applet: `dx build --destination mtsg:/mtsg-$VERSION dnanexus/mtsg`
  * [ ] Publish DNAnexus app: `dx build --app --publish dnanexus/mtsg`
  * [ ] Build St. Jude Cloud production workflow on DNAnexus.
  * [ ] Download DNAnexus-built workflow: `dx get $DXID`
  * [ ] Build DNAnexus workflow: `dx build --workflow --destination "$DX_PROJECT_NAME:$DX_PATH"`
