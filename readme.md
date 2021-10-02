# @live/swc-wallaby

Returns `ranges` as needed by Wallaby custom compilers.

```diff
- const {code, map} = transformSync(...)
+ const {code, map, ranges} = transformSync(...)
```

See: https://wallabyjs.com/docs/config/compilers.html#writing-a-custom-compiler

# Why

Parsing AST with SWC and sending to JS is currently slow in SWC.

The JS-based parser/visitor is being deprecated, it will be Rust-only in future.

When using a JS-based visitor, the spans (locations) are raw byte positions without line number which need processing. Also, on consecutive calls, the byte positions begin from the end of the previous file.

# Reference

See: https://github.com/wallabyjs/public/issues/2823

Project adapted from: https://github.com/vercel/next.js/tree/canary/packages/next/build/swc
