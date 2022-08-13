# rarity-ranker
An implicit way of defining rarities for each trait in an NFT collection. Written in Rust so most OS's can run the binaries, making it very accessible and fast.

The executable in https://github.com/TheBigMort/rarity-ranker/blob/main/rarity-ranker/rarity-ranker can be used in MacOS. To use it, create a folder and download the executable into the folder and create 2 folders, "config" and "in". All of your traits into the "in" folder. The program categorizes your traits based on the folder they are located in. For example, put your "head" traits into a folder named "head"(placed in the "in" folder).

If you have issues with the binary or dont feel like using it you can clone this repo, run "cargo b --release" (assuming you have rustup installed) and continue with the steps above.
