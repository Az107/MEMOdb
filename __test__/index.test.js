const {MEMOdb} = require("./index.js");


test('MEMOdb is a class', () => {
  expect(typeof MEMOdb).toBe("function");
  const memodb = new MEMOdb();
  expect(MEMOdb.version).toBe("v0.02");
});


