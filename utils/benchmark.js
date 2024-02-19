const {MEMOdb} = require("../index.js");

const memodb = new MEMOdb();

function benchmark_write(value) {
  const repetitions = Math.pow(10, value);
  console.time("Write");
  memodb.create("test");
  const collection = memodb.get("test");
  for (let i = 0; i < repetitions; i++) {
    collection.add({name: "test", index: i});
  }
  console.timeEnd("Write");
}

function benchmark_wirte_random(value) {
  const repetitions = Math.pow(10, value);
  console.time("Write Random");
  memodb.create("random");
  const collection = memodb.get("random");
  for (let i = 0; i < repetitions; i++) {
    collection.add({name: "test", index: Math.random()});
  }
  console.timeEnd("Write Random");
}

function benchmark_read(value) {
  const repetitions = Math.pow(10, value);
  console.time("Read");
  const collection = memodb.get("test");
  for (let i = 0; i < repetitions; i++) {
    collection.get(i);
  }
  console.timeEnd("Read");
}

benchmark_write(5);
//benchmark_wirte_random();
//benchmark_read();