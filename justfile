critique_random username *args:
    cargo run --bin critique_random -- {{username}} {{args}}
critique2boxd username *args:
    cargo run --bin critique2boxd -- {{username}} {{args}}
publish_website:
    rm -rf docs && mkdir docs
    cd critique_website && dx bundle --out-dir ../docs
    mv docs/public/* docs
    cp docs/index.html docs/404.html
