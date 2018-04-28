build() {
    cd frontend/index || exit
    cargo web build --target wasm32-unknown-unknown --release || exit
    cd ../.. || exit
    cp frontend/index/target/wasm32-unknown-unknown/release/index.wasm static/ || exit
    cp frontend/index/target/wasm32-unknown-unknown/release/index.js static/ || exit
    cargo build
}

run() {
    cargo run
}

case "$1" in
    build)
        build
        ;;
    run)
        build
        run
        ;;
    *)
        echo "Usage <builder.sh build|run>"
        ;;
esac
