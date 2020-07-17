FROM r-base:4.0.2 AS env

RUN apt-get update \
      && apt-get --yes install --no-install-recommends \
        libcurl4-openssl-dev \
        libssl-dev \
        libxml2-dev \
      && rm -r /var/lib/apt/lists/*

RUN echo 'options(repos = "https://cloud.r-project.org/", Ncpus = parallel::detectCores()); \
        install.packages("BiocManager"); \
        BiocManager::install(c( \
            "MutationalPatterns", \
            "BSgenome", \
            "BSgenome.Hsapiens.UCSC.hg19", \
            "BSgenome.Hsapiens.UCSC.hg38", \
            "rtracklayer", \
            "GenomicRanges" \
        ), version = "3.11")' | R --vanilla

FROM rust:1.45.0-buster AS app

COPY .git/ /app/.git/
COPY Cargo.lock Cargo.toml /app/
COPY src/ /app/src/
COPY test/ /app/test/

RUN cargo build --release --manifest-path /app/Cargo.toml

FROM env

COPY --from=app /app/target/release/mtsg /opt/mtsg/bin/

ENTRYPOINT ["/opt/mtsg/bin/mtsg"]
