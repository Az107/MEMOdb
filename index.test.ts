import MEMdb from "./src";

test("dumb test", () => {
    console.log("ok");
    expect(true).toBe(true);
})

test("create test", () => {
   const mem = MEMdb.getInstance();
   expect(mem).toBeDefined();
});

test("create collection", ()=> {
    const  mem = MEMdb.getInstance();
    mem.addCollection("test");
    expect(mem.get("test")).toBeDefined();
});

test("add item to collection", () => {
    const mem = MEMdb.getInstance();
    const oldLength = mem.get("test").length;
    const result = mem.get("test").push("test");
    expect(result).toBe(oldLength + 1);
});


test("dump file", () => {
    const mem = MEMdb.getInstance();
    mem.dump();
});

beforeAll(() => {
});