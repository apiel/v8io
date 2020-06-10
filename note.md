By default the app take no parameter. It will always try to execute `bootstrap.js`. An env variable might be define to determine the path of this one, else it should take the `bootstrap.js` located next to his instalation folder. The boostrap take care to load all the plugin, without those plugin the app cannot make any interaction outside v8 (no write, no reat, no internet...). Bootstrap will also take care to load the standard library from node, deno or whatever... It will as well take care to setup the module resolver. Through the bootstrap, we can finally have a logic to parse the parameter and decide if we start a script or do something else like showing installation or ...

Import, by default it will look into file for relations... so we can push stuff into the load. maybe setModuleResolver should have multiple options, replace completely the loader, or be only used as fallback, or as well can be used inside the custom resolver

- links:
  - current path: https://doc.rust-lang.org/std/env/fn.current_exe.html
  - https://learning-rust.github.io/docs/d3.modules.html

- use plugin for FunctionTemplate call as `core.`
    - `usePlugin(__driname + 'fs.so', { some: 'variables'})`
        - plugin should return a list of available function
        - plugin should return a type definition
    - `freezePlugins()` would not allow to load plugin anymore
    - links:
        - https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html
        - https://docs.rs/libloading/0.5.0/libloading/
        - http://adventures.michaelfbryan.com/posts/plugins-in-rust/

- module
    - should resolve to the given file as fallback by default
    - `setModuleResolver((specifier: string, referrer: string) => string)` would overwrite previous and resolve with a js function
    - should we also allow to resolve with a plugin?

- default core function:
    - print
    - version
