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
const Collection_1 = __importDefault(require("./Collection"));
const fs_1 = require("fs");
class MEMOdb {
    constructor() {
        this.PATH = "./MEM.json";
        this.data = new Map();
        //this.load()
    }
    static getInstance() {
        if (!this.instace) {
            this.instace = new MEMOdb();
        }
        return this.instace;
    }
    addCollection(name) {
        let collection = new Collection_1.default(this);
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
        if (this.data.has(name))
            this.data.delete(name);
    }
    dump(path = this.PATH) {
        const replacer = (key, value) => {
            if (value instanceof Map) {
                return { '__map__': Array.from(value.entries()) };
            }
            return value;
        };
        let jsonData = JSON.stringify(this.data, replacer);
        fs.writeFileSync(path, jsonData);
    }
    load(path = this.PATH) {
        if (!(0, fs_1.existsSync)(path))
            this.dump(path);
        let jsonData = fs.readFileSync(path, 'utf-8');
        const reviver = (key, value) => {
            if (typeof value === 'object' && value !== null && '__map__' in value) {
                return new Map(value['__map__']);
            }
            return value;
        };
        this.data = JSON.parse(jsonData, reviver);
        if (!(this.data instanceof Map))
            throw new Error("invalid data!");
    }
}
exports.default = MEMOdb;
MEMOdb.version = "v0.01";
