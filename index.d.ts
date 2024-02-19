// index.d.ts
type memoDocument = Map<string, any>

type Collection = {
    name: string

    
}

declare module 'index.js' {
    export class MEMOdb {
        constructor()
        version: string
        public create(name: string, data: any): void
        public get(name: string): string
        public list(): string[]
        public getAll(): Collection[]
        public remove(name: string): void
    
    }
}

export default MEMOdb;