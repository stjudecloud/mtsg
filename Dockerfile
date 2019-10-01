FROM r-base:3.6.1 AS env

RUN apt-get update \
      && apt-get --yes install --no-install-recommends \
        libcurl4-openssl-dev \
        libssl-dev \
        libxml2-dev \
      && rm -r /var/lib/apt/lists/*

RUN echo 'install.packages("BiocManager", repos = "https://cloud.r-project.org/"); \
        BiocManager::install(c( \
            "MutationalPatterns", \
            "BSgenome", \
            "BSgenome.Hsapiens.UCSC.hg19", \
            "BSgenome.Hsapiens.UCSC.hg38", \
            "rtracklayer", \
            "GenomicRanges" \
        ), version = "3.9")' | R --vanilla

FROM rust:1.38.0 AS app

COPY Cargo.lock Cargo.toml /app/
COPY src/ /app/src/
COPY test/ /app/test/

RUN cargo build --release --manifest-path /app/Cargo.toml

FROM env

COPY --from=app /app/target/release/mtsg /opt/mtsg/bin/

ENTRYPOINT ["/opt/mtsg/bin/mtsg"]
