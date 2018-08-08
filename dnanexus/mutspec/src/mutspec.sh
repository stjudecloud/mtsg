#!/usr/bin/env bash

main() {
    set -ex

    DATA_DIR=$HOME/data
    RESULTS_DIR=$HOME/results

    mkdir -p $DATA_DIR $RESULTS_DIR/vcfs

    VCF_FILENAME=$(dx describe --name "$multi_sample_vcf")
    dx download --output $DATA_DIR/$VCF_FILENAME "$multi_sample_vcf"

    dx-docker run \
        --volume $DATA_DIR:/data \
        --volume $RESULTS_DIR:/results \
        mutspec \
        --verbose \
        split-vcf \
        --output-directory /results/vcfs \
        /data/$VCF_FILENAME

    if [[ -z "$sample_sheet" ]]; then
        dx-docker run \
            --volume $RESULTS_DIR:/results \
            mutspec \
            --verbose \
            generate-sample-sheet \
            --output /results/sample-sheet.txt \
            /results/vcfs
    else
        dx download --output $RESULTS_DIR/sample-sheet.txt "$sample_sheet"
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
        /results/sample-sheet.txt

    dx-docker run \
        --volume $RESULTS_DIR:/results \
        mutspec \
        --verbose \
        visualize \
        --output /results/signatures.html \
        /results/signatures.txt

    signatures_txt=$(dx upload --brief $RESULTS_DIR/signatures.txt)
    signatures_html=$(dx upload --brief $RESULTS_DIR/signatures.html)

    dx-jobutil-add-output --class file signatures_txt "$signatures_txt"
    dx-jobutil-add-output --class file signatures_html "$signatures_html"
}
