import MEMOdb from "./src/index";

// test("Dummy test", () => {
//     expect(true).toBe(true);
// })

test("create", () => {
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
    const result = mem.get("test").push({"test": 0});
    expect(result).toBe(oldLength + 1);
});

test("get item by id", () => {
    const mem = MEMOdb.getInstance();
    const id = mem.get("test").add({"test":1});
    const result = mem.get("test").getById(id);
    expect(result).toBeDefined();
});

test("get all items", () => {
    const mem = MEMOdb.getInstance();
    const result = mem.get("test").getAll();
    expect(result).toEqual([{"test":0},{"test":1}]);
});


test("dump file", () => {
    const mem = MEMOdb.getInstance();
    mem.dump();
});

test("load file", () => {
    const mem = MEMOdb.getInstance();
    mem.load();
});

beforeAll(() => {
});

export { };
