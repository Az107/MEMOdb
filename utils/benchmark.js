const {MEMOdb} = require("../index.js");

const memodb = new MEMOdb();


//make a baisc benchmark with a callback 
function benchmark(value, callback) {
  console.log(`Benchmark write with ${value} repetitions.`);
  const repetitions = Math.pow(10, value);
  console.time("Benchmark");
  callback(repetitions);
  console.timeEnd("Benchmark");
}

function benchmark_write(repetitions) {
  const memodb = new MEMOdb();
  memodb.create("test");
  const collection = memodb.get("test");
  for (let i = 0; i < repetitions; i++) {
    collection.add({name: "test", index: i});
  }
}

function benchmark_wirte_random(repetitions) {
  memodb.create("random");
  const collection = memodb.get("random");
  for (let i = 0; i < repetitions; i++) {
    collection.add({name: "test", index: Math.random()});
  }
}

function benchmark_read(value) {
  console.log(`Benchmark read with ${value} repetitions.`);
  const repetitions = Math.pow(10, value);
  console.time("Read");
  const collection = memodb.get("test");
  for (let i = 0; i < repetitions; i++) {
    collection.get(i);
  }
  console.timeEnd("Read");
}

benchmark(5, benchmark_write);
//benchmark_wirte_random();
//benchmark_read();