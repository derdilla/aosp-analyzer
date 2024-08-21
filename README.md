Script to calculate the size of the AOSP source code. For more meaning behind 
the data, check out the blog post(TODO).

For the version used in the [first blog post](https://derdilla.com/blog/size-aosp.html), look [here](https://github.com/NobodyForNothing/aosp-analyzer/tree/0e31f9099ba5c913aa06b7bba192aa231c27ddb0).

## Usage

1. Download full android source: `bash download.sh` 
2. Use [tokei](https://github.com/XAMPPRocky/tokei) to get detailed per-file stats: `bash analyze.sh`
3. Visualize result
