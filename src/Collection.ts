import MEMdb from "./index.js";
import { randomUUID } from "crypto";

export class Collection<T> {
    private data: Array<CollectionItem<T>> = [];
    private context: MEMdb;

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
 

    constructor(context: MEMdb) {
        this.context = context;
    }

}

type CollectionItem<T> = T & {
    ID: string
}




