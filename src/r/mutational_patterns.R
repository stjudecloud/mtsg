library("MutationalPatterns")
library("BSgenome")
library("rtracklayer")
library("GenomicRanges")

args <- commandArgs(trailingOnly = TRUE)
vcfs_dir <- args[1]
sample_sheet_in <- args[2]
signatures_in <- args[3]
genome_build <- args[4]
min_burden <- as.numeric(args[5])
min_contribution <- as.numeric(args[6])
out_dir <- args[7]

load(signatures_in)

ref_genome <- if (genome_build == "GRCh37") {
  "BSgenome.Hsapiens.UCSC.hg19"
} else {
  "BSgenome.Hsapiens.UCSC.hg38"
}

library(ref_genome, character.only = TRUE)

message("Reading input VCFs")
vcf_pathnames <- list.files(path = vcfs_dir, pattern = ".vcf", full.names = TRUE)
sample_names <- tools::file_path_sans_ext(basename(vcf_pathnames))
vcfs <- read_vcfs_as_granges(vcf_pathnames, sample_names, ref_genome)

message("Filtering VCFs (mitochondrial and allosomal chromosomes)")
autosomes <- extractSeqlevelsByGroup(species = "Homo_sapiens", style = "UCSC", group = "auto")
filtered_vcfs <- lapply(vcfs, function(x) keepSeqlevels(x, autosomes, pruning.mode = "coarse"))

message("Filtering VCFs (mutational burden threshold)")
mutation_counts <- elementNROWS(filtered_vcfs)
filtered_vcfs <- filtered_vcfs[mutation_counts > min_burden]

message("Building mutation matrix")
mutation_matrix <- mut_matrix(vcf_list = filtered_vcfs, ref_genome = ref_genome)
write.table(
  mutation_matrix,
  file = file.path(out_dir, "mutation_matrix.txt"),
  sep = "\t",
  quote = FALSE
)

message("Reading sample sheet")
sample_sheet <- read.table(sample_sheet_in, header = FALSE)
colnames(sample_sheet) <- c("ID", "tissue")

message("Matching samples to tissue origins")
filtered_sample_names <- data.frame(names(filtered_vcfs))
colnames(filtered_sample_names) <- "ID"
tissues <- merge(filtered_sample_names, sample_sheet)

message("Counting occurences of base substitution types")
type_context <- type_context(vcfs[[1]], ref_genome)
type_occurrences <- mut_type_occurrences(filtered_vcfs, ref_genome)

message("Plotting mutation spectrum")
pdf(file = file.path(out_dir, "summary.pdf"), width = 10)
plot_spectrum(type_occurrences)
plot_spectrum(type_occurrences, CT = TRUE)
plot_spectrum(type_occurrences, by = tissues$tissue, CT = FALSE, legend = TRUE)
invisible(dev.off())

message("Plotting trinucleotide profiles")
pdf(file = file.path(out_dir, "cosmic_signatures.pdf"), width = 7, height = 30)
plot_96_profile(cancer_signatures[,1:30])
invisible(dev.off())

message("Fitting mutation matrix to signature matrix")
fit_result <- fit_to_signatures(mutation_matrix, cancer_signatures)
contribution <- fit_result$contribution
filtered_contributions <- contribution[rowSums(contribution) > min_contribution,]
signatures <- as.data.frame(t(filtered_contributions))
signatures$tissue <- tissues$tissue
write.table(
  signatures,
  file = file.path(out_dir, "signatures.txt"),
  sep = "\t",
  quote = FALSE
)
