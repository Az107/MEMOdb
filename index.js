const _memodb = require('./index.node');

//MEMOdb js wrapper

export default class MEMOdb {
  static version = "v0.02";
  instance = null;

  constructor() {
    this.instance = new _memodb.new();
  }

  get(name) {
    return this.instance.getCollection(this.instance, name);
  }

  create(name) {
    return this.instance.createCollection(this.instance, name);
  }


}