import * as fs from "fs";
import { Collection } from "./Collection";

export default class MEMdb {
  static instace: MEMdb;
  private constructor(){
    this.load()
  }

  static getInstance() {
    if (!this.instace) {
      this.instace = new MEMdb();
    }
    return this.instace;
  }

  private PATH = "./MEM.json";
  private data:any = {};

  addCollection<T>(name:string){
    let collection = new Collection<T>(this);
    this.data[name] = collection;
  }

  get<T>(name:string): Array<T> {
    name = name.toLowerCase();
    if (!this.data[name]) {
      throw new Error("collection do not exist");
    }
    this.data[name]
    return this.data[name];
  }


  delCollection(name:string) {
    this.data[name] = undefined;
  }


  dumb(path:string = this.PATH) {
    let jsonData = JSON.stringify(this.data);
    fs.writeFileSync(path,jsonData);
  }

  load(path:string = this.PATH) {
    console.log("load file")
    let jsonData = fs.readFileSync(path,'utf-8');
    this.data = JSON.parse(jsonData);
  }

}

class compressor {

  static compressObject(object: Object) {
    // iterate object properties
    Object.keys(object).forEach(property => {
      //create new array of ids
      let ids = [];
      //check if property is and array
      if (Array.isArray(property)){
        property.map((e) => {
          //check if element in the array has a property id
          if (e["id"]) {
            //add id to ids array
            ids.push(e["id"]);
          }
        });
        object
      }

    });
  }

}
