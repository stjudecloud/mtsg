#!/usr/bin/env bash

main() {
    set -ex

    DATA_DIR=$HOME/data
    RESULTS_DIR=$HOME/results

    mkdir -p $DATA_DIR $RESULTS_DIR/vcfs

    VCF=$(dx describe --name "$multi_sample_vcf")

    if [[ -z "$prefix" ]]; then
        PREFIX=$(basename $(basename $VCF .vcf) .vcf.gz)
    else
        PREFIX=$prefix
    fi

    SAMPLE_SHEET=$PREFIX.sample.sheet.txt
    SIGNATURES_HTML=$PREFIX.signatures.html
    SIGNATURES_TXT=signatures.txt

    dx download --output $DATA_DIR/$VCF "$multi_sample_vcf"

    dx-docker run \
        --volume $DATA_DIR:/data \
        --volume $RESULTS_DIR:/results \
        mutspec \
        --verbose \
        split-vcf \
        --output-directory /results/vcfs \
        /data/$VCF

    if [[ -z "$sample_sheet" ]]; then
        dx-docker run \
            --volume $RESULTS_DIR:/results \
            mutspec \
            --verbose \
            generate-sample-sheet \
            --output /results/$SAMPLE_SHEET \
            /results/vcfs
    else
        dx download --output $RESULTS_DIR/$SAMPLE_SHEET "$sample_sheet"
    fi

    dx-docker run \
        --volume $RESULTS_DIR:/results \
        mutspec \
        --verbose \
        run \
        --output-directory /results \
        --genome-build $genome_build \
        --min-burden $min_burden \
        --min-contribution $min_contribution \
        /results/vcfs \
        /results/$SAMPLE_SHEET

    dx-docker run \
        --volume $RESULTS_DIR:/results \
        mutspec \
        --verbose \
        visualize \
        --output /results/$SIGNATURES_HTML \
        /results/$SIGNATURES_TXT

    signatures_txt=$(dx upload --brief $RESULTS_DIR/$SIGNATURES_TXT)
    signatures_html=$(dx upload --brief $RESULTS_DIR/$SIGNATURES_HTML)
    sample_sheet_out=$(dx upload --brief $RESULTS_DIR/$SAMPLE_SHEET)

    dx-jobutil-add-output --class file signatures_txt "$signatures_txt"
    dx-jobutil-add-output --class file signatures_html "$signatures_html"
    dx-jobutil-add-output --class file sample_sheet_out "$sample_sheet_out"
}
