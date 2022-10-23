import MEMOdb from "./index";
import { randomUUID } from "crypto";

export default class Collection<T> {
    private data: Array<CollectionItem<T>> = [];
    private context: MEMOdb;

    public length = 0;

    push(...items: T[]): number {
        items.forEach(item => {
            const collectionItem: CollectionItem<T> = {
               ...item,
               ID: ""
            };
            this.data.push(collectionItem)
        });
        this.length = this.data.length;
        return this.data.length;
    }


    getById(id: string): T | undefined {
        const result = this.data.find(item => item.ID == id);
        return result as T;
    }
 

    constructor(context: MEMOdb) {
        this.context = context;
    }

}

type CollectionItem<T> = T & {
    ID: string
}




