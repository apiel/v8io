# v8io

## import module

The default module loader is very basic and only allow to import files with relative path. Also, unlike node.js, you must specify the extension of the file.

```js
import "./bootchild.js";

print(`hello world\n`);
```

## core api

Only few native functions are available by default. All other native functions must be loaded with plugin.

- Use `print(text: string)` only for the primary output of your program.
- Use `eprint(text: string)` only for error and progress messages.
