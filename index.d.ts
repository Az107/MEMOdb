// index.d.ts
type memoDocument = Map<string, any>

type Collection = {
    name: string
    data: memoDocument[]
}

declare module 'index.js' {
    export class MEMOdb {
        constructor()
        public get(name: string): string
        public list(): string[]
        public getAll(): Collection[]
    }
}