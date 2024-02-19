const {MEMOdb} = require("../index.js");

const memodb = new MEMOdb();

function benchmark_write() {
  console.time("Write");
  memodb.create("test");
  const collection = memodb.get("test");
  for (let i = 0; i < 10000; i++) {
    collection.add({name: "test", index: i});
  }
  console.timeEnd("Write");
}

function benchmark_wirte_random() {
  console.time("Write Random");
  memodb.create("random");
  const collection = memodb.get("random");
  for (let i = 0; i < 10000; i++) {
    collection.add({name: "test", index: Math.random()});
  }
  console.timeEnd("Write Random");
}

function benchmark_read() {
  console.time("Read");
  const collection = memodb.get("test");
  for (let i = 0; i < 10000; i++) {
    collection.get(i);
  }
  console.timeEnd("Read");
}

benchmark_write();
benchmark_wirte_random();
//benchmark_read();