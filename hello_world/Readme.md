## Create Project
```
Cargo init hello_world
```
this will create hello_world folder
* src/ 
* Cargo.toml

## Build Project
```
cd hello_world
cargo build
```
this step will generate the targe folder. the generated binary file locate at:
* target/debug/hello_world


now you can run the hello world bin:
```
./target/debug/hellow_world
```

## Run Project
This is an alternative step for carge build. Instead of build. User can also run this:
```
cargo run 
```

cargo run can compile and run the project, providing a more convenience way

## Other cargo command
```
# check cargo version
cargo --version

# check project before build
cargo check
```
