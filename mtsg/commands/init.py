from pathlib import Path

from SigProfilerMatrixGenerator.install import install

from mtsg import GenomeBuild


def init(genome_build: GenomeBuild) -> None:
    install(str(genome_build))
