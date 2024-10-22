Script to calculate the size of the AOSP source code. For more meaning behind 
the data, check out the [blog post](https://derdilla.com/blog/size-aosp14.html).

For the version used in the [first blog post](https://derdilla.com/blog/size-aosp.html), look [here](https://github.com/derdilla/aosp-analyzer/tree/0e31f9099ba5c913aa06b7bba192aa231c27ddb0).

## Usage

Each of the tree steps is independend from another, so you can quickly see the results of modifications. If you just want to run your own visualization of the data you can get the result of step 2 by unpackging `sample-stats.tar.lzama` (AOSP main branch on 9 Jun 2024).

1. Download full android source: `bash download.sh` (This **will** take a while, so depending on your internet speed make a lunch break or go to sleep. This operation takes ~100GB of disk space.)
2. Use [tokei](https://github.com/XAMPPRocky/tokei) to get detailed per-file stats: `bash analyze.sh` (Tokei is fairly fast, but the AOSP is large so expect 10 minutes to an hour depending on your hardware.)
3. `cargo run` the visualizer to generate a index.html (very fast)

*note: The stats directory should be next to the cwd executing the analyzer (../stats). The source code is the intended place for further customization.*

## TODO

- Try to use [vega](https://github.com/vega/vega) for stats
- Add percentage of language stats with toggleable categories
  - Area of the code: Core, SDKs, Third-party, ...
  - Code kind: Code, comments, blank
- Allow generating time series
- Some sample historical data
