# NIST ACVP-Server gen/val tools (https://github.com/usnistgov/ACVP-Server).
#
# Published as ghcr.io/celabshq/libcrux-acvp-server:<tag> by .github/workflows/docker-acvp-server.yml,
# where <tag> is the git tree hash of this directory (`git rev-parse HEAD:.docker/acvp-server`).
# `cargo xtask acvp` pulls that image, and builds it when it is not published.
#
# Generate test vectors from a registration; the outputs (prompt.json,
# expectedResults.json, internalProjection.json) land next to each registration:
# `docker run --rm -v "$PWD":/work <image> generate /work/<dir>/registration.json ...`

FROM mcr.microsoft.com/dotnet/sdk:8.0@sha256:6ba9644da07bb5b560a402acec9ca38711a0762969f1b61da261df5c9f6f85da AS build

# Release v1.1.0.43. Keep in sync with crates/testing/kats/Readme.md.
ARG ACVP_SERVER_COMMIT=975de31eb83d87039ec88934fdc47d8c312b892d

WORKDIR /acvp
RUN git init -q . \
 && git remote add origin https://github.com/usnistgov/ACVP-Server.git \
 && git sparse-checkout set --no-cone '/_config/' '/gen-val/src/' '/gen-val/samples/' \
 && git fetch -q --depth 1 --filter=blob:none origin "$ACVP_SERVER_COMMIT" \
 && git checkout -q FETCH_HEAD \
 && ln -s ./_config/Directory.Build.props Directory.Build.props \
 && ln -s ./_config/Directory.Packages.props Directory.Packages.props \
 && dotnet publish gen-val/samples/NIST.CVP.ACVTS.Orleans.ServerHost/NIST.CVP.ACVTS.Orleans.ServerHost.csproj -c Release -o /out/server \
 && dotnet publish gen-val/samples/GenValAppRunner/src/NIST.CVP.ACVTS.Generation.GenValApp.csproj -c Release -o /out/genval

FROM mcr.microsoft.com/dotnet/aspnet:8.0@sha256:ea721098d6ea2894ff9bd8387821a1659d4765d6c014b8d648bb1da16f016f5a
LABEL maintainer="CE Labs <info@celabs.eu>"
LABEL org.opencontainers.image.source="https://github.com/celabshq/libcrux"

COPY --from=build /out /out
COPY entrypoint.sh /entrypoint.sh
# The container runs as the user owning the mounted work directory, see `cargo xtask acvp`.
ENV HOME=/tmp
ENTRYPOINT ["/bin/bash", "/entrypoint.sh"]
