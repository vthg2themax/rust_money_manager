#Setup the project directory
project_directory=~/Nextcloud/Projects/money_manager
cd $project_directory
#Build the WebAssembly File
cargo build --target wasm32-unknown-unknown --release
#Get to the release directory to continue
cd target/wasm32-unknown-unknown/release/
#Generate the [b]ind[g]en version of the webassembly
wasm-bindgen --target web --no-typescript --out-dir . money_manager.wasm
#Shake the tree of dead leaves to give it a smaller size
wasm-gc money_manager_bg.wasm 

#Ensure the directories exist for the website
mkdir -p $project_directory/www/scripts
mkdir -p $project_directory/www/css

#Copy the webassembly scripts to the www directory
cp -v money_manager_bg.wasm $project_directory/www/scripts/
cp -v money_manager.js $project_directory/www/scripts/
#Copy the standard html,css, and js files to the www directory
cd $project_directory
cp -rv src/scripts/ www/
cp -rv src/css/ www/
cp -v src/index.html www/