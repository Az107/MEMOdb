const _memodb = require('./index.node');

//MEMOdb js wrapper
 class MEMOdb {
  static version = "v0.02";
  instance = null;

  constructor() {
    this.instance = _memodb.new();
  }

  list() {
    return this.instance.getCollectionList(this.instance);
  }

  getAll() {
    return this.instance.getAll(this.instance);
  }

  get(name) {
    return this.instance.getCollection(this.instance, name);
  }

  create(name) {
    return this.instance.createCollection(this.instance, name);
  }

}

exports.MEMOdb = MEMOdb;