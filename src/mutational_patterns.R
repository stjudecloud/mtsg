suppressMessages(library("MutationalPatterns"))
suppressMessages(library("BSgenome"))
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
prefix <- args[8]

ref_genome <- if (genome_build == "GRCh37") {
  "BSgenome.Hsapiens.UCSC.hg19"
} else {
  "BSgenome.Hsapiens.UCSC.hg38"
}

library(ref_genome, character.only = TRUE)

vcf_pathnames <- list.files(path = vcfs_dir, pattern = ".vcf", full.names = TRUE)
sample_names <- tools::file_path_sans_ext(basename(vcf_pathnames))

message(sprintf("Reading %d input VCFs", length(vcf_pathnames)))
vcfs <- read_vcfs_as_granges(vcf_pathnames, sample_names, ref_genome)

message("Filtering VCFs (mitochondrial and allosomal chromosomes)")
autosomes <- extractSeqlevelsByGroup(species = "Homo_sapiens", style = "UCSC", group = "auto")
filtered_vcfs <- lapply(vcfs, function(x) keepSeqlevels(x, autosomes, pruning.mode = "coarse"))

message("Filtering VCFs (mutational burden threshold)")
mutation_counts <- elementNROWS(filtered_vcfs)
filtered_vcfs <- filtered_vcfs[mutation_counts > min_burden]

message(sprintf("Building mutation matrix from %d VCFs", length(filtered_vcfs)))
mutation_matrix <- mut_matrix(vcf_list = filtered_vcfs, ref_genome = ref_genome)
filename <- paste(prefix, "mutation_matrix.txt", sep = ".")
write.table(
  mutation_matrix,
  file = file.path(out_dir, filename),
  sep = "\t",
  quote = FALSE,
  col.names = NA
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
filename <- paste(prefix, "summary.pdf", sep = ".")
pdf(file = file.path(out_dir, filename), width = 10)
plot_spectrum(type_occurrences)
plot_spectrum(type_occurrences, CT = TRUE)
plot_spectrum(type_occurrences, by = tissues$tissue, CT = FALSE, legend = TRUE)
invisible(dev.off())

message("Loading COSMIC signature probabilities")
cancer_signatures <- as.matrix(read.table(
  signatures_in,
  sep = "\t",
  header = TRUE,
  row.names = 1
))

message("Plotting trinucleotide profiles")
pdf(file = file.path(out_dir, "cosmic-signatures.pdf"), width = 7, height = 30)
plot_96_profile(cancer_signatures[,1:30])
invisible(dev.off())

message("Fitting mutation matrix to signature matrix")
fit_result <- fit_to_signatures(mutation_matrix, cancer_signatures)
contribution <- fit_result$contribution

message("Filtering contributions")
filtered_contributions <- contribution[rowSums(contribution) > min_contribution,]
signatures <- as.data.frame(t(filtered_contributions))
signatures$tissue <- tissues$tissue
filename <- paste(prefix, "signatures.txt", sep = ".")
write.table(
  signatures,
  file = file.path(out_dir, filename),
  sep = "\t",
  quote = FALSE,
  col.names = NA
)
