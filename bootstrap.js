import "./bootchild.js";
import "./bootchild.js";

const delay = t => new Promise(resolve => setTimeout(resolve, t));

async function yo2() {
  print("hello" + "world" + Math.random() + "\n");

  // await = import("./bootchild.js");
}
yo2();

print(`hello yeah ${yo()}\n`);

import("./bootchild.js").then(() => {
  print(`dyn import done\n`);
});
