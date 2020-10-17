**Note**: this release has a reproducible build issue. When building as `root` inside Docker, as
`build.sh` does, the build is slightly different to when it is built as `dockeruser`. It is still
reproducible using `scripts/dockerenv.sh`. In any case, `v9.2.1` replaces this release, fixing this
issue.
