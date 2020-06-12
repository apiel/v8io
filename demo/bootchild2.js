import "./bootchild.js";

let val = 456;
print(`bootchild2 ${val}\n`);

export function child2() {
    return "child2";
}

import('./bootchild3.js').then(({ child3 }) => {
    print(`bootchild3 loaded ${ child3() }\n`);
});
