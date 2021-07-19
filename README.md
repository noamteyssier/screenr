# SCREENR
This is a tool for parsing CRISPRi/a screens and assigned counts to the provided guides

## Installation
```bash
# clone repo and install binary
git clone https://github.com/noamteyssier/screenr
cd screenr
cargo install --release

# validate screenr is in your path
screenr --version

# look at help menu
screenr --help
```

## Usage
```bash
# perform count mapping for a single fastq
screenr \
	--input data/test/subset0000.fastq.gz \
	--name lib1 \
	--output lib1.tab \
	--library data/libraries/CRISPRi_v2_crop28.all.fasta.gz

# perform count mapping for multiple fastqs
screenr \
	--input data/test/subset0000.fastq.gz data/test/subset0001.fastq.gz data/test/subset0002.fastq.gz \
	--name lib1 lib2 lib3 \
	--output sample_counts_libs123.tab \
	--library data/libraries/CRISPRi_v2_crop28.all.fasta.gz

# perform count mapping for all fastqs in a directory
screenr \
	--input data/test/subset00*.fastq.gz \
	--name lib{0..10} \
	--output sample_counts.tab \
	--library data/libraries/CRISPRi_v2_crop28.all.fasta.gz

# perform count mapping for all fastqs in a directory only using a subset of sgRNAs
screenr \
	--input data/test/subset00*.fastq.gz \
	--name lib{0..10} \
	--output sample_counts.tab \
	--library data/libraries/CRISPRi_v2_crop28.1.fasta.gz
```
