"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const fs = __importStar(require("fs"));
const Collection_js_1 = __importDefault(require("./Collection.js"));
const fs_1 = require("fs");
class MEMOdb {
    constructor() {
        this.PATH = "./MEM.json";
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
    addCollection(name) {
        let collection = new Collection_js_1.default(this);
        this.data.set(name, collection);
    }
    get(name) {
        name = name.toLowerCase();
        if (!this.data.get(name)) {
            throw new Error("collection do not exist");
        }
        return this.data.get(name);
    }
    list() {
        return Array.from(this.data.keys());
    }
    delCollection(name) {
        this.data.delete(name);
    }
    dump(path = this.PATH) {
        let jsonData = JSON.stringify(this.data);
        fs.writeFileSync(path, jsonData);
    }
    load(path = this.PATH) {
        console.log("load file");
        if (!(0, fs_1.existsSync)(path))
            fs.writeFileSync(path, "{}");
        let jsonData = fs.readFileSync(path, 'utf-8');
        this.data = JSON.parse(jsonData);
    }
}
exports.default = MEMOdb;
MEMOdb.version = "v0.01";
class compressor {
    static compressObject(object) {
        // iterate object properties
        Object.keys(object).forEach(property => {
            //create new array of ids
            let ids = [];
            //check if property is and array
            if (Array.isArray(property)) {
                property.map((e) => {
                    //check if element in the array has a property id
                    if (e["id"]) {
                        //add id to ids array
                        ids.push(e["id"]);
                    }
                });
                object;
            }
        });
    }
}
