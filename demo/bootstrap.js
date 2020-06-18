// import "core_freeze_plugins";
import "../../plugins/demo/target/debug/libhello.so";

import "./bootchild.js";
import "./bootchild.js";
import "try_array";

// setModuleLoader();

print(`args ${JSON.stringify(args())}\n`);

export function coreModuleLoader() {
  print(`call "coreModuleLoader"\n`);
  return 10;
}
export let coreYo = 123;

// const res = coreInstantiate("hello", "hello");
// print(`coreInstantiate res: ${res}\n`);

const res = hello("world");
print(`hello res: ${res}\n`);

// async function yo2() {
//   print("hello" + "world" + Math.random() + "\n");

//   const { child2 } = await import("./bootchild2.js");

//   print(`yo2 dyn import done ${ child2() }\n`);
// }
// yo2();

async function callYo() {
  print(`hello yeah ${await coreTest()}\n`);

  const res2 = await coreInstantiateAsync("adder", "hello");
  print(`coreInstantiateAsync res: ${res2}\n`);
}
callYo();

// print(`hello yeah ${coreTest()}\n`);

import("./bootchild.js").then(({ child }) => {
  print(`dyn import done ${child()}\n`);
});

import("./bootchild.js").then(({ child }) => {
  print(`dyn2 import done ${child()}\n`);
});
