/**
 * Generate a random integer in the given **inclusive** range.
 */
 export const getRandomInteger = (min: number, max: number): number => {
  const [intMin, intMax] = [Math.ceil(min), Math.floor(max)];
  return Math.floor(Math.random() * (intMax - intMin + 1)) + intMin;
};
