# stage 1: builder

FROM ubuntu:18.04 as builder

RUN apt-get update \
    && apt-get -y install \
        build-essential \
        curl \
        # reqwest
        libssl-dev \
        pkg-config \
    && rm -r /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
ENV PATH=/root/.cargo/bin:$PATH

COPY Cargo.lock /app/Cargo.lock
COPY Cargo.toml /app/Cargo.toml
COPY src /app/src

RUN cargo build --manifest-path /app/Cargo.toml --release

# stage 2: main

FROM ubuntu:18.04

# Set the timezone before installing r-base to avoid having to interact with tzdata.
RUN ln -fs /usr/share/zoneinfo/UTC /etc/localtime \
    && apt-get update \
    && apt-get -y install \
        r-base \
        libxml2-dev \
        # curl (R lib)
        libcurl4-openssl-dev \
        libssl-dev \
        # RMySQL
        libmariadb-client-lgpl-dev \
    && rm -r /var/lib/apt/lists/*

RUN echo 'source("https://bioconductor.org/biocLite.R"); \
        biocLite(c( \
            "MutationalPatterns", \
            "BSgenome", \
            "BSgenome.Hsapiens.UCSC.hg19", \
            "BSgenome.Hsapiens.UCSC.hg38", \
            "rtracklayer", \
            "GenomicRanges" \
        ))' | R --vanilla

COPY --from=builder /app/target/release/mutspec /app/bin/
COPY r/ /app/src/r/

ENTRYPOINT ["/app/bin/mutspec"]
