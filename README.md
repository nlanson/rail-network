### WORK IN PROGRESS
# 🚄 Rail-net
```rail-network``` is a program that creates and renders fictional rail maps. It does this by using random numbers and a sprinkle of math to create networks of stations and routes and then renders them onto your screen using `(nannou)[https://github.com/nannou-org/nannou]`.


## Running the program
> ⚠️rail-network is very early in development and may not work yet as intended. Please see `docs/docs.md` for details.

Currently, to run Rail-net you will need to build from source. 
### Step 1: Install Rust
You can download the Rust compiler and package manager, Cargo, from `https://www.rust-lang.org/tools/install`.

### Step 2: Clone this repository
> Run this in in an empty directory where you want to store the source files for this repository.

`git clone https://github.com/nlanson/rail-network.git`

### Step 3: Build and Run
> Run this in the directory that you cloned this repository into:

```bash
# This command will build and run the project straight away
cargo run

# Use this command to create an executable instead (Found under ./target)
cargo build --release
```