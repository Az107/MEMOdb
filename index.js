const _memodb = require('./MEMOdb.node');

//MEMOdb js wrapper
class MEMOdb {
  static version = "v0.03";
  instance = null;

  constructor() {
    this.instance = _memodb.new();
  }

  list() {
    return _memodb.getCollectionList(this.instance);
  }

  getAll() {
    return this.list().map((collection) => {
      return new Collection(collection, this.instance);
    });
  }

  get(name) {
    return new Collection(name, this.instance);
  }

  create(name) {
    return _memodb.createCollection(this.instance, name);
  }

  dump() {
    console.log("Not implemented yet");
  }

}

class Collection {
    constructor(name, dbInstance) {
        this.name = name;
        this.instance = dbInstance;
    }

    add(data) {
        _memodb.collectionAdd(this.instance, this.name, data);
    }

    get(index) {
        return _memodb.collectionGet(this.instance, this.name, index);
    }

    getAll() {
        return _memodb.collectionGetAll(this.instance, this.name);
    }

    remove(index) {
        _memodb.collectionRm(this.instance, this.name, index);
    }

    count() {
        return _memodb.collectionCount(this.instance, this.name);
    }

}

exports.MEMOdb = MEMOdb;