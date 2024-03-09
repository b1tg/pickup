# Pickup

[中文文档](./README_CN.md)

Copy files to clipboard from command line, expecting to achieve the same effect as right-click copy. Now support Windows, OSX and Linux.


# How to use

## Build

Step 0: [install rust](https://rustup.rs/)

Step 1: clone & build 

```sh
git clone https://github.com/b1tg/pickup.git
cd pickup
cargo build --release
# you can find pickup or pickup.exe in ./target/release/
```

Step 2 (optional): add  `pickup`  to you PATH

Step 3 (only for linux): install xclip


## Usage

```
Usage: pickup [file patterns]

Exameple: 
    # copy single file
    pickup Cargo.toml
    
    # copy all pdf file in target subdirectories (need double quotation)
    pickup "docs/**/*.pdf"


    # copy folder
    pickup src
```
## Demo

### On Windows:

![demo on windows](./images/pickup-demo-win.gif)

### On OSX:
![demo on osx](./images/pickup-demo-osx.gif)



# References

- https://stackoverflow.com/q/25708895
- https://github.com/roryyorke/picellif
- https://github.com/yujinqiu/pbadd