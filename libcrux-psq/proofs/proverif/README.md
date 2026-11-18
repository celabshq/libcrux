# PSQ ProVerif analysis

This folder contains the symbolic security analysis of the PSQ
handshake based on models extracted from the implementation with the
hax toolchain. The models are extracted with the `check-pv.sh`, which
also patches some extraction artifacts and runs the ProVerif analyses
on the extracted models.

Since the full analysis is very resource intensive, we only run
reachability checks on CI (`.github/workflows/psq-proverif.yml`) as a
sanity check on the extracted models.


## Running the full analysis with Docker

The image in `.docker/hax` has hax 0.5.0 and ProVerif. Run these
commands from the repository root.

```sh
# Build the image
docker build -t hax:0.5.0 --build-arg HAX_VERSION=0.5.0 .docker/hax

# Extract and run the analysis
docker run --rm -it -v "$PWD":/work hax:0.5.0 \
  bash libcrux-psq/proofs/proverif/check-pv.sh
```

The container runs as user `opam` (uid 1000). It needs write access to the
checkout for the extraction output, the logs, and the Rust build directory
`target/docker-hax`.

The script exits with 0 if every analysis matched, 1 on a wrong verdict or
model error, and 2 if nothing failed but a run timed out or ran out of memory.
Per-analysis logs and `summary.txt` go to
`libcrux-psq/proofs/proverif/pv-logs/<timestamp>/`.

## Useful options

Pass these with `-e`, e.g. `docker run -e SKIP_SLOW=1 ...`.

| Variable                | Effect                                                                   |
|-------------------------|--------------------------------------------------------------------------|
| `LIST=1`                | List the analyses and exit (add `NO_EXTRACT=1` to skip extraction first) |
| `ONLY=a,b` / `SKIP=a,b` | Run or skip analyses by name or glob, e.g. `ONLY='reg-*-msg1'`           |
| `SKIP_SLOW=1`           | Skip the two registration session analyses (about 1 h and 40 min)        |
| `PV_SERIAL=1`           | Run the slow analyses one after another to lower peak memory             |
| `PV_TIMEOUT=<s>`        | Wall-clock limit per ProVerif run (default 14400)                        |
| `NO_EXTRACT=1`          | Verify the extraction already on disk instead of re-extracting           |
| `EXTRACT_ONLY=1`        | Extract and post-process, but do not run ProVerif                        |
| `PV_RECHECK=1`          | Judge the logs in `PV_LOG_DIR` again without running ProVerif            |

The `check-pv.sh` script can read a memory limit on the ProVerif
process from `PV_MEM_LIMIT`.  In Docker, the container has no systemd, so
`PV_MEM_LIMIT` has no effect, and an out-of-memory kill can stop
the whole script. Cap memory with `docker run --memory=<limit>`
instead, and use `PV_SERIAL=1` on machines with less memory.

The CI subset is
`ONLY='reg-passive-sanity,session-*-passive'`.
