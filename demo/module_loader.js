let yo = 0; // to demonstrate that it keep memory

function coreModuleLoader(specifier, referrer) {
  // print(`coreModuleLoader (${++yo}) ${specifier} ${referrer}\n`);
  const ret =
    !specifier.endsWith(".js") &&
    `${referrer.substr(0, referrer.lastIndexOf("/") - 1)}${specifier}.js`;
  // print(`coreModuleLoader (${++yo}) ${ret}\n`);
  return ret;
}
