let yo = 0; // to demonstrate that it keep memory

function coreModuleLoader(specifier, referrer) {
  print(`coreModuleLoader (${++yo}) ${specifier} ${referrer}\n`);
  return specifier.endsWith('.js') ? specifier : `${specifier}.js`;
}
