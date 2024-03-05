import {MEMOdb} from '../index.js';
import test from 'ava'

const user = {
  name: 'test',
  age: 20
}

test('MEMOdb is a class', (t) => {
  t.is(typeof MEMOdb, 'function');
  const memodb = new MEMOdb();
  //memodb is defined and has
  t.is(typeof memodb, 'object');
});

//test create Collection  
test('create Collection', (t) => {
  const memodb = new MEMOdb();
  memodb.createCollection('test');
  const collection = memodb.get('test');
  t.is(typeof collection, 'object');
  t.is(collection.name, 'test');
});

//test write and read
test('write and read', (t) => {
  const memodb = new MEMOdb();
  memodb.createCollection('test');
  const collection = memodb.get('test');
  let id = collection.add(user)
  let user2 = collection.get(id);
  t.deepEqual(user, user2);
});

