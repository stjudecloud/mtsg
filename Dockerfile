# stage 1

FROM ubuntu:18.04 AS env

# Add repository for R 3.6.
RUN apt-get update \
    && apt-get -y --no-install-recommends install ca-certificates gnupg \
    && echo "deb https://cloud.r-project.org/bin/linux/ubuntu bionic-cran35/" > /etc/apt/sources.list.d/r.list \
    && apt-key adv --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys E298A3A825C0D65DFD57CBB651716619E084DAB9

# Set the timezone before installing r-base to avoid having to interact with tzdata.
RUN ln -fs /usr/share/zoneinfo/UTC /etc/localtime \
    && apt-get update \
    && apt-get -y --no-install-recommends install \
        build-essential \
        r-base \
        libxml2-dev \
        # curl (R lib)
        libcurl4-openssl-dev \
        libssl-dev \
        # htslib
        libbz2-dev \
        liblzma-dev \
        # RMySQL
        libmariadb-client-lgpl-dev \
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

# stage 2

FROM env AS app

RUN apt-get update \
    && apt-get -y --no-install-recommends install \
        build-essential \
        curl \
        # reqwest
        libssl-dev \
        pkg-config \
    && rm -r /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
ENV PATH=/root/.cargo/bin:$PATH

# Cache project dependencies.
RUN USER=root cargo new --vcs none /app
COPY Cargo.lock Cargo.toml /app/
RUN cargo build --release --manifest-path /app/Cargo.toml && rm -r /app/src

COPY src /app/src
COPY test /app/test

RUN cargo build --release --manifest-path /app/Cargo.toml

# stage 3

FROM env

COPY --from=app /app/target/release/mtsg /opt/mtsg/bin/

ENTRYPOINT ["/opt/mtsg/bin/mtsg"]
