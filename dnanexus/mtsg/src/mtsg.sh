#!/usr/bin/env bash

main() {
    set -ex

    DATA_PREFIX=$HOME/data
    RESULTS_PREFIX=$HOME/results

    zstd -T0 -d -c --no-progress $RESOURCES/tmp/mtsg-latest.tar.zst | docker load

    mkdir -p $DATA_PREFIX $RESULTS_PREFIX

    dx-download-all-inputs --parallel
    mv $HOME/in/vcf_srcs/**/* $DATA_PREFIX

    if [[ $(find $DATA_PREFIX -name "*.vcf.gz" -print -quit) ]]; then
        gzip --decompress $DATA_PREFIX/*.vcf.gz
    fi

    docker run \
        --mount type=bind,source=$DATA_PREFIX,target=/data \
        --mount type=bind,source=$RESULTS_PREFIX,target=/results \
        mtsg \
        run \
        --genome-build $genome_build \
        --dst-prefix /results \
        /data

    docker run \
        --mount type=bind,source=$RESULTS_PREFIX,target=/results \
        mtsg \
        visualize \
        --output /results/Sig_activities.html \
        /results/Sig_activities.txt

    signature_activities_txt=$(dx upload --brief $RESULTS_PREFIX/Sig_activities.txt)
    signature_activities_html=$(dx upload --brief $RESULTS_PREFIX/Sig_activities.html)

    dx-jobutil-add-output --class file signature_activities_txt "$signature_activities_txt"
    dx-jobutil-add-output --class file signature_activities_html "$signature_activities_html"
}
