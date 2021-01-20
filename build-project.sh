#Setup the project directory
project_directory=~/Nextcloud/Projects/money_manager
#Some commands to help build
cd $project_directory
#Ensure the directories exist
mkdir -p www/scripts
#Build the WebAssembly File
cargo build --target wasm32-unknown-unknown --release
#Get to the release directory to continue
cd target/wasm32-unknown-unknown/release/
#Generate the [b]ind[g]en version of the webassembly
wasm-bindgen --target web --no-typescript --out-dir . money_manager.wasm
#Shake the tree of dead leaves to give it a smaller size
wasm-gc money_manager_bg.wasm 
#Copy it to the www directory
cp -v money_manager_bg.wasm ../../../www/scripts/
cp -v money_manager.js ../../../www/scripts/
#Go back to the project directory
cd $project_directory
cp -rv src/scripts www/scripts
cp -v src/index.html www/