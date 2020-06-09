- use plugin for FunctionTemplate
    print() plugins/print.so
    https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html
    https://docs.rs/libloading/0.5.0/libloading/
    http://adventures.michaelfbryan.com/posts/plugins-in-rust/

- module
    - should resolve as fallback by default
    - should look for lib/module.js (or this be set in cli)
      if exist it will be responsable for module resolution
    - `setModuleResolver(specifier: string, referrer: string)` would overwrite previous

- cli
    - should fallback as executing the script passed as first param
    - should look for lib/cli.js (should this be call boostrap.js)
      if return string execute script
