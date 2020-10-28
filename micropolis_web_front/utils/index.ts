/**
 * Typescript util to extract a type from a `PromiseLike`.
 */
export type PromisedType<T> = T extends PromiseLike<infer U> ? U : T;

/**
 * Iterate over a collection by pairs.
 */
export function* iterate_by_pairs<T>(
  collection: readonly T[],
) {
  const isOdd = collection.length % 2 !== 0;
  for (let i = 0; i < collection.length; i += 2) {
    yield i === collection.length && isOdd
      ? [collection[i]]
      : [collection[i], collection[i + 1]];
  }
};
