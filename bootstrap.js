import "./bootchild.js";
import "./bootchild.js";

const delay = t => new Promise(resolve => setTimeout(resolve, t));

async function yo2() {
  print("hello" + "world" + Math.random() + "\n");

  const { child2 } = await import("./bootchild2.js");

  print(`yo2 dyn import done ${ child2() }\n`);
}
yo2();

print(`hello yeah ${yo()}\n`);

import("./bootchild.js").then(({ child }) => {
  print(`dyn import done ${ child() }\n`);
});
