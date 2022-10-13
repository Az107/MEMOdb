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
Object.defineProperty(exports, "__esModule", { value: true });
const fs = __importStar(require("fs"));
class MEMdb {
    static instace;
    constructor() {
        this.load();
    }
    static getInstance() {
        if (!this.instace) {
            this.instace = new MEMdb();
        }
        return this.instace;
    }
    PATH = "./MEM.json";
    data = {};
    addCollection(name) {
        let collection = [];
        this.data[name] = collection;
    }
    get(name) {
        name = name.toLowerCase();
        if (!this.data[name]) {
            throw new Error("collection do not exist");
        }
        return this.data[name];
    }
    delCollection(name) {
        this.data[name] = undefined;
    }
    dumb(path = this.PATH) {
        let jsonData = JSON.stringify(this.data);
        fs.writeFileSync(path, jsonData);
    }
    load(path = this.PATH) {
        console.log("load file");
        let jsonData = fs.readFileSync(path, 'utf-8');
        this.data = JSON.parse(jsonData);
    }
}
exports.default = MEMdb;
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
