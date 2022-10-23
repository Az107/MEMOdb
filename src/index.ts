import * as fs from "fs";
import Collection  from "./Collection";
import { existsSync } from "fs";

export default class MEMOdb {
  static version = "v0.01";
  static instace: MEMOdb;
  private constructor(){
    this.data = new Map();
    //this.load()
  }

  static getInstance() {
    console.log("getting memodb instance");
    if (!this.instace) {
      this.instace = new MEMOdb();
    }
    return this.instace;
  }

  private PATH = "./MEM.json";
  private data: Map<string,Collection<any>>;

  addCollection<T>(name:string){
    let collection = new Collection<T>(this);
    this.data.set(name,collection);
  }

  get<T>(name:string): Collection<T> {
    name = name.toLowerCase();
    if (!this.data.get(name)) {
      throw new Error("collection do not exist");
    }
    return this.data.get(name) as Collection<T>;
  }

  list(): string[] {
    return Array.from(this.data.keys());
  }

  delCollection(name:string) {
    this.data.delete(name);
  }

  dump(path:string = this.PATH) {
    let jsonData = JSON.stringify(this.data);
    fs.writeFileSync(path,jsonData);
  }

  load(path:string = this.PATH) {
    console.log("load file");
    if (!existsSync(path)) fs.writeFileSync(path,"{}");
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

