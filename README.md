# Introduction

De-Block, aka, anti-block. Use this project to remove block words in your literature work, to prevent the falsify of your chapters.

The rules are as follow:

1. Scan block words in Chinese, and put corresponding pinyin.
2. Scan block words in English, and put space into letters.

This project is under construction.

# Usage

The core is `de-block.py` file, you can use this file separately.

All other files are belongs to the back-end or front-end. Currently, are not finished.

```bash
python3 -m pip install pypinyin
python3 de-block <directory>
```

The `<directory>` means a folder that you puts all the text files into it.

Output files would replace the origin one. **Please backup your files before use.**

# Back-end

Back-end is based on the `Rust` language and `Rocket` framework.

Install Rust before you use the back-end.

```bash
rustup default nightly
cargo run
```

# Comments & Issues

If you have any comments or issues to this project, feel free to push an issue.

Any update of block word list can also use a `pull request` to update the origin one.

Please attach this project information if you use any source code from here.

# Authors

@EnderaoeLyther

@RR