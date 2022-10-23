import MEMOdb from "./src/index";

test("dumb test", () => {
    console.log("ok");
    expect(true).toBe(true);
})

test("create test", () => {
   const mem = MEMOdb.getInstance();
   expect(mem).toBeDefined();
});

test("create collection", ()=> {
    const  mem = MEMOdb.getInstance();
    mem.addCollection("test");
    expect(mem.get("test")).toBeDefined();
});

test("add item to collection", () => {
    const mem = MEMOdb.getInstance();
    const oldLength = mem.get("test").length;
    const result = mem.get("test").push("test");
    expect(result).toBe(oldLength + 1);
});


test("dump file", () => {
    const mem = MEMOdb.getInstance();
    mem.dump();
});

beforeAll(() => {
});

export { };
