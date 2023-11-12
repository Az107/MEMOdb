import MEMOdb from "./index";
import { randomUUID } from "crypto";

export default class Collection<T> {
    private data: Array<CollectionItem<T>> = [];
    //private context: MEMOdb;

    public length = 0;

    push(...items: T[]): number {
        
        items.forEach(item => {
            this.add(item);
        });
        // this.length = this.data.length;
        return this.data.length;
    }


    add(item: T, id?: ID): ID {
        // for (const property in item ) {
        //     //item[property] = this.add<>(property);
        // }
        const collectionItem: CollectionItem<T> = {
            ...item,
            ID: id || randomUUID(),
            getOriginal: () => getOriginal(collectionItem)
         };
         this.data.push(collectionItem)
         this.length = this.data.length;
         return collectionItem.ID;
         
    }

    getById(id: string): CollectionItem<T> | undefined {
        const result = this.data.find(item => item.ID == id);
        return result;
    }

    getAll(): T[] {
        return this.data.map(item => item.getOriginal());
    }
 

    constructor(context: MEMOdb) {
        //this.context = context;
    }

}

type CollectionItem<T> = T & {
    ID: ID,
    getOriginal: () => T
}

const getOriginal = <T>(item: CollectionItem<T>): T => {
    const { ID, ...original } = item;
    return original as T;
}

type ID = string;


