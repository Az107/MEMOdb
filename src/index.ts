import * as fs from "fs";
import Collection  from "./Collection";
import { existsSync } from "fs";

export default class MEMOdb {
  static version = "v0.02";
  static instace: MEMOdb;
  private constructor(){
    this.data = new Map();
    //this.load()
  }

  static getInstance() {
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
    if (this.data.has(name))
      this.data.delete(name);
  }

  dump(path:string = this.PATH) {
    const replacer = (key: string, value: any) => {
      if (value instanceof Map) {
        return { '__map__': Array.from(value.entries()) };
      }
      return value;
    };
    let jsonData = JSON.stringify(this.data,replacer);
    fs.writeFileSync(path,jsonData);
  }

  load(path:string = this.PATH) {
    
    if (!existsSync(path)) this.dump(path);
    let jsonData = fs.readFileSync(path,'utf-8');
    const reviver = (key: string, value: any) => {
      if (typeof value === 'object' && value !== null && '__map__' in value) {
        let map = new Map(value['__map__']);
        // convert each map value to a collection
        map.forEach((v: any, k: any) => {
          let collection = new Collection(this);
          v.data.forEach((item:any) => {
            collection.add(item,item.ID);
          });
          map.set(k,collection);
        });
        return map;
      }
      return value;
    };
    this.data =  JSON.parse(jsonData,reviver);
    if (!(this.data instanceof Map)) throw new Error("invalid data!");
  }

}