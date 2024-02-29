const _m = require("../MEMOdb.node");
let m = new _m.MEMOdb();
let user = {name: "Alb", age: 24}
let user2 = {name: "Enya", age: 20}
m.createCollection("test");
m.getCollection("Test").add(user)
m.getCollection("Test").add(user)
m.getCollection("Test").get(1)