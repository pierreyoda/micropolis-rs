/**
 * Iterate over a collection by pairs.
 */
export function* iterate_by_pairs<T>(collection: readonly T[]) {
  const isOdd = collection.length % 2 !== 0;
  for (let i = 0; i < collection.length; i += 2) {
    yield i === collection.length && isOdd ? [collection[i]] : [collection[i], collection[i + 1]];
  }
}

/**
 * Generate a random integer in the given **inclusive** range.
 */
export const getRandomInt = (min: number, max: number): number => {
  const [intMin, intMax] = [Math.ceil(min), Math.floor(max)];
  return Math.floor(Math.random() * (intMax - intMin + 1)) + intMin;
};
