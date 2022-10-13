import MEMdb from ".";
import {randomUUID} from "crypto";

export class Collection<T> {
    private data: Array<CollectionItem<T>> = [];
    private context: MEMdb;

    push(...items: T[]): number {
        items.forEach(item => {
            const collectionItem: CollectionItem<T> = {
               ...item,
               ID: ""
            };
            this.data.push(collectionItem)
        });
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




