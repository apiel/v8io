# V8io

Ryan Dahl created Node.js and about 10 years later Deno. Deno is kind of improvment of Node.js from an architecture perspective. I got very interested about this new project and started to develop some modules. Unfortunately, since it is an early project, it is still very limited and for the moment very opinitated to their vision. I didn't wanted to be stuck and just decided to dive into the code, to make some feature request. But the code is hugeeee (at least for a Rust beginner)! and I didn't manage to achieve anything. Therefor, I decided to first try to understand [Rusty_v8](https://github.com/denoland/rusty_v8) before to go futher with Deno, this is why I created v8io. V8io, is intended to be a very basic implementation of a server side JavaScript engine, containing the strict minimum features to run JS on the server. With the time, I decided to make it as configurable as possible, to be able to gradually add extra features. The idea, would be to make a JS engine, fully customizable, corresponding only to the minium requierement to run an application.

## Bootstrap

V8io is very basic and reduce to the strict minimum to be customizable. V8io doesn't get any parameter, when it is instaciated, it will always try to run `./boostrap.js` that is located next to his executable installation folder (not where you instanciate it). Of course, is it not that's strict, we can as well use the environment variable `V8IO_BOOSTRAP` to provide the absolute path of the bootstrap script.

This boostrap is intented to setup the working environment. In this file, we will decide which plugin to load, how to resolve the modules and this will be as well the place to parse the params passed during instantiation. We will then decide if we run an application or if we do some devops operations like installing a module.

## Import module

The default module loader is very basic and only allow to import files with relative path. Also, unlike node.js, we must specify the extension of the file.

```js
import "./bootchild.js";

print(`hello world\n`);
```

### Custom module loader

To be implemented:
    - `setModuleLoader((specifier: string, referrer: string) => string)` would default loader and resolve with a js function
    - should we also allow to resolve with a native plugin?

## Core api

Only few native functions are available by default. All other native functions must be loaded with plugin.

- Use `print(text: string)` only for the primary output of your program.
- Use `eprint(text: string)` only for error and progress messages.

## Plugin

To provide more native feature to v8io, we need to use plugins (shared library .dll, .so, etc).

To be implemented:
    - `usePlugin(__driname + 'fs.so', { some: 'variables'})`
        - plugin should return a list of available function
        - plugin should return a type definition
    - `freezePlugins()` would not allow to load plugin anymore
