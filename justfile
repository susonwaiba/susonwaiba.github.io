list:
    @just --list

install:
    npm install

serve:
    zola serve

watch-style:
    npm run watch

build:
    npm run build
    zola build --base-url https://susonwaiba.github.io
