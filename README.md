# SCREENR
This is a tool for parsing CRISPRi/a screens and assigned counts to the provided guides

## Installation
```bash
# clone repo and install binary
git clone https://github.com/noamteyssier/screenr
cd screenr
cargo install --path . 

# validate screenr is in your path
screenr --version

# look at help menu
screenr --help
```

## Usage
### Count mapping for a single sample
```bash
# perform count mapping for a single fastq
screenr \
	--input data/example/subset0000.fastq.gz \
	--names lib1 \
	--library data/libraries/CRISPRi_v2_crop28.all.fasta.gz
```

### Count mapping for a single sample writing results to an output file
```bash
# perform count mapping for a single fastq
screenr \
	--input data/example/subset0000.fastq.gz \
	--names lib1 \
	--library data/libraries/CRISPRi_v2_crop28.all.fasta.gz
	--output lib1.tab
```

### Count mapping for multiple samples
```bash
screenr \
	--input data/example/subset0000.fastq.gz data/example/subset0001.fastq.gz data/example/subset0002.fastq.gz \
	--names lib1 lib2 lib3 \
	--library data/libraries/CRISPRi_v2_crop28.all.fasta.gz
```


### Count mapping for all samples in a directory
```bash
screenr \
	--input data/example/subset00*.fastq.gz \
	--names lib{0..10} \
	--library data/libraries/CRISPRi_v2_crop28.all.fasta.gz
```

### Count mapping for all fastqs in a directory only using a subset of sgRNAs
```bash
screenr \
	--input data/example/subset00*.fastq.gz \
	--names lib{0..10} \
	--library data/libraries/CRISPRi_v2_crop28.1.fasta.gz
```

### Count mapping for all fastqs in a directory using a custom search guide 
```bash
# default = "GTTTAAGAG"
screenr \
	--input data/example/subset00*.fastq.gz \
	--names lib{0..10} \
	--library data/libraries/CRISPRi_v2_crop28.1.fasta.gz
	--guide GCGCGAA
```
