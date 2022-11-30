#Temp
cargo install wasm-pack
#Building the package
#wasm-pack build --target web --release
#In short, wasm-pack build:
# 1. Compiles your Rust code to WebAssembly.
# 2. Runs wasm-bindgen on that WebAssembly, generating a JavaScript file that wraps up that WebAssembly file into a module the browser can understand.
# 3. Creates a pkg directory and moves that JavaScript file and your WebAssembly code into it.
# 4. Reads your Cargo.toml and produces an equivalent package.json.
# 5. Copies your README.md (if you have one) into the package.

#gzip -9 < pkg/wasm_game_of_life_bg.wasm
#you can also gzip it further


##Run this command to enable the building of our beloved wasm
#rustup target add wasm32-unknown-unknown
##Install SSL to make sure this next process can continue
#sudo apt install libssl-dev
##Ensure this tool is installed as well
#cargo install wasm-bindgen-cli
##Ensure this garbage collector is installed
#cargo install wasm-gc


#Build the WebAssembly File
wasm-pack build --target web --release
#Setup the project directory
project_directory=~/Nextcloud/Projects/money_manager
cd $project_directory
#cargo build --target wasm32-unknown-unknown --release
#Get to the release directory to continue
#cd target/wasm32-unknown-unknown/release/
#Generate the [b]ind[g]en version of the webassembly
#wasm-bindgen --target web --no-typescript --out-dir . money_manager.wasm
#Shake the tree of dead leaves to give it a smaller size
#wasm-gc money_manager_bg.wasm 

#Ensure the directories exist for the website
mkdir -p $project_directory/target/www/scripts
mkdir -p $project_directory/target/www/css

#Copy the webassembly scripts to the www directory
cp -v $project_directory/pkg/money_manager_bg.wasm $project_directory/target/www/scripts/
cp -v $project_directory/pkg/money_manager.js $project_directory/target/www/scripts/
#Copy the standard html,css, and js files to the www directory
cd $project_directory
cp -rv src/scripts/ target/www/
cp -rv src/css/ target/www/
cp -v src/index.html target/www/