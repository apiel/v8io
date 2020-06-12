import "./bootchild3.js";

let val = 123;
print(`bootchild ${val}\n`);

export function child() {
    return "child";
}
