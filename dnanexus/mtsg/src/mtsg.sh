#!/usr/bin/env bash

main() {
    set -ex

    DEFAULT_PREFIX=mtsg
    MTSG_SPLIT_VCF_EXTRA_ARGS=""

    DATA_DIR=$HOME/data
    RESULTS_DIR=$HOME/results

    mkdir -p $DATA_DIR $RESULTS_DIR/vcfs

    dx-download-all-inputs --parallel --except sample_sheet
    mv $HOME/in/vcfs/**/* $DATA_DIR

    if [[ -z "$prefix"  ]]; then
        if [[ ${#vcfs[@]} -eq 1 ]]; then
            VCF=$(dx describe --name "${vcfs[0]}")
            PREFIX=$(basename "$(basename "$VCF" .vcf)" .vcf.gz)
        else
            PREFIX=$DEFAULT_PREFIX
        fi
    else
        PREFIX=$prefix
    fi

    if [[ ! -z "$disable_column" ]]; then
        MTSG_SPLIT_VCF_EXTRA_ARGS="--disable-column $disable_column"
    fi

    SAMPLE_SHEET="$PREFIX.sample.sheet.txt"
    SIGNATURES_HTML="$PREFIX.signatures.html"
    SIGNATURES_TXT="$PREFIX.signatures.txt"

    dx-docker run \
        --volume $DATA_DIR:/data \
        --volume $RESULTS_DIR:/results \
        --entrypoint /bin/bash \
        mtsg \
        -c \
        "/opt/mtsg/bin/mtsg \
        --verbose \
        split-vcf \
        --output-directory /results/vcfs \
        $MTSG_SPLIT_VCF_EXTRA_ARGS \
        /data/*.vcf*"

    if [[ -z "$sample_sheet" ]]; then
        dx-docker run \
            --volume $RESULTS_DIR:/results \
            mtsg \
            --verbose \
            generate-sample-sheet \
            --output "/results/$SAMPLE_SHEET" \
            /results/vcfs
    else
        dx download --output "$RESULTS_DIR/$SAMPLE_SHEET" "$sample_sheet"
    fi

    sample_sheet_out=$(dx upload --brief "$RESULTS_DIR/$SAMPLE_SHEET")

    dx-docker run \
        --volume $RESULTS_DIR:/results \
        mtsg \
        --verbose \
        run \
        --output-directory /results \
        --prefix "$PREFIX" \
        --genome-build $genome_build \
        --min-burden $min_burden \
        --min-contribution $min_contribution \
        /results/vcfs \
        "/results/$SAMPLE_SHEET"

    dx-docker run \
        --volume $RESULTS_DIR:/results \
        mtsg \
        --verbose \
        visualize \
        --output "/results/$SIGNATURES_HTML" \
        "results/$SIGNATURES_TXT"

    signatures_txt=$(dx upload --brief "$RESULTS_DIR/$SIGNATURES_TXT")
    signatures_html=$(dx upload --brief "$RESULTS_DIR/$SIGNATURES_HTML")

    dx-jobutil-add-output --class file signatures_txt "$signatures_txt"
    dx-jobutil-add-output --class file signatures_html "$signatures_html"
    dx-jobutil-add-output --class file sample_sheet_out "$sample_sheet_out"
}
