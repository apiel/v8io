# V8io

Ryan Dahl created Node.js and about 10 years later Deno. Deno is kind of improvment of Node.js from an architecture perspective. I got very interested about this new project and started to develop some modules. Unfortunately, since it is an early project, it is still very limited and for the moment very opinitated to their vision. I didn't wanted to be stuck and just decided to dive into the code, to make some feature request. But the code is hugeeee (at least for a Rust beginner)! and I didn't manage to achieve anything. Therefor, I decided to first try to understand [Rusty_v8](https://github.com/denoland/rusty_v8) before to go futher with Deno, this is why I created v8io. V8io, is intended to be a very basic implementation of a server side JavaScript engine, containing the strict minimum features to run JS on the server. With the time, I decided to make it as configurable as possible, to be able to gradually add extra features. The idea, would be to make a JS engine, fully customizable, corresponding only to the minium requierement to run an application.

## Bootstrap

V8io is very basic and reduce to the strict minimum to be customizable. V8io doesn't get any parameter, when it is instaciated, it will always try to run `./boostrap.js` that is located next to his executable installation folder (not where you instanciate it). Of course, is it not that's strict, we can as well use an environment variable `V8IO_BOOSTRAP` to provide the absolute path of the bootstrap script.

This boostrap is intented to setup the working environment. In this file, we will decide which plugin to load and we will parse the params passed during instantiation. We will then decide if we run an application or if we do some devops operations like installing a module.

## Import module

The default module loader is very basic and only allow to import files with relative path. Also, unlike Node.js, we must specify the extension of the file.

```js
import "./module.js";

print(`hello world\n`);
```

To load module dynamically, like `require()` in Node.js, we can use dynamic import.

```js
import('./module.js').then(({ default }) => {
    print(`module loaded ${ default() }\n`);
});
```

Unlike dynamic import in Deno or `require()` in Node.js, the module will always be instantiated and not use the cache. We can change this behavior by using a custom module loader.

### Custom module loader

To define your own loader, create a file called `module_loader.js`, in the same folder as the v8io executable, as we did for `bootstrap.js`. We can as well use an environment variable `V8IO_MODULE_LOADER` to provide the absolute path of the module loader script.

This script should contain a function `coreModuleLoader(specifier: string, referrer: string) => string | [string, string] | undefined` and return a string to the absolute path of the module to load. We can as well return an array, where the first element is the path of the module and the second element is the source code of the module. If something else is returned, then it will fallback to the default module loader.

```js
// example return a string
function coreModuleLoader(specifier, referrer) {
  return (
    !specifier.endsWith(".js") &&
    `${referrer.substr(0, referrer.lastIndexOf("/") - 1)}${specifier}.js`
  );
}
```

It would add `.js` to any path without extension, `import "./hello"` would become `/absolute/path/hello.js`.

```js
// example return an array
function coreModuleLoader(specifier, referrer) {
  if (specifier === "array_example") {
    return ["/array_example.js", 'print("this is array_example code.\\n");'];
  }
}
```

It would then display `this is array_example code.` if we have `import "array_example";` in our code.

> **Note**: It might be important to provide the right path to the module, because child module might use it as referrer.

## Core api

Only few native functions are available by default. All other native functions must be loaded with plugin.

- Use `print(text: string)` only for the primary output of your program.
- Use `eprint(text: string)` only for error and progress messages.
- Use `getArgs()` to be implemented.

## Plugin

To provide more native feature to v8io, we use plugins (shared library .dll, .so, etc). In your bootstrap file, import plugins as you would import a JS module: `import "./my_plugin.so"`. Finally when all plugins as been imported, call `import "core_freeze_plugins"`, so no more plugins can be imported by third party module.

```js
import "./core/libfs.so";
import "./core/libpath.so";
import "core_freeze_plugins";

// ... then bootstrap
```

### Build your own plugin

In this example, we are building a plugin where we can call `hello('world')` returning `some data to return` and writing as output `run: world`:

```rs
// here we define the name of the plugin, so we can call it with coreInstantiate('hello')
#[no_mangle]
fn get_name() -> String {
    "hello".to_string()
}

// here is some JavaScript code to execute to initialize the plugin
#[no_mangle]
fn get_code() -> String {
    "globalThis.hello = (value) => coreInstantiate('hello', value);".to_string()
}

// here is a synchrone function
#[no_mangle]
fn run(params_str: &str) -> Option<String> {
    println!("run {:?}", params_str);
    // can only return a string
    Some("some data to return".to_string())
}

// here is a asynchrone function
#[no_mangle]
fn run_async(params_str: &str, cb: Box<dyn FnMut(Option<String>)>) {
    println!("run {:?}", params_str);
    let mut cb = cb;
    // can only return a string
    cb(Some("some data to return".to_string()));
}
```

The cargo file:
```toml
[package]
name = "hello"
version = "0.0.1"
edition = "2018"

[lib]
path = "hello.rs"
crate-type = ["cdylib"]
```
