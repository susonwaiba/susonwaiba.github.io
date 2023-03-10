# Portfolio Repository

URL: https://susonwaiba.github.io

My personal website project.

## Frontend setup

https://github.com/leptos-rs/leptos
https://trunkrs.dev

```
trunk serve
```

## Json generator

```
cd md_to_json
cargo run
```

## Deployment

```
trunk build --release

cp dist/index.html dist/404.html
cp -r dist docs
```

Copy `index.html` to `404.html` to use github 404 page overwrite feature.

Copy `dist` to `docs` dir to deploy with default github page action.

Dir `dist` is in .gitignore.

Copyright © Suson Waiba. All rights reserved.
