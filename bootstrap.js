import "./bootchild.js";
import "./bootchild.js";

setModuleLoader();

export function coreModuleLoader() {
  print(`call "coreModuleLoader"\n`);
  return 10;
}
export let coreYo = 123;

// async function yo2() {
//   print("hello" + "world" + Math.random() + "\n");

//   const { child2 } = await import("./bootchild2.js");

//   print(`yo2 dyn import done ${ child2() }\n`);
// }
// yo2();

// print(`hello yeah ${yo()}\n`);

import("./bootchild.js").then(({ child }) => {
  print(`dyn import done ${ child() }\n`);
});
