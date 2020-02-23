export type PromisedType<T> = T extends Promise<infer U> ? U : T;
