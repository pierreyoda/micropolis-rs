import { iterate_by_pairs } from "../utils";

describe("Utils test suite", () => {
  it("should properly implement iterate_by_pairs", () => {
    const collection = [...Array(5).keys()];
    const iter = iterate_by_pairs(collection);
    expect(iter.next().value).toEqual([0, 1]);
    expect(iter.next().value).toEqual([2, 3]);
    expect(iter.next().value).toEqual([4, undefined]);
  });
});
