import MEMdb from "./src";

test("dumb test", () => {
    console.log("ok");
    expect(true).toBe(true);
})

test("create test", () => {
   const mem = MEMdb.getInstance();
   expect(mem).toBeDefined();
});