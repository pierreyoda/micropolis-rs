import { iterateByPairs } from "../utils";

describe("Utils test suite", () => {
  it("should properly implement iterate_by_pairs", () => {
    const collection = [...Array(5).keys()];
    const iter = iterateByPairs(collection);
    expect(iter.next().value).toEqual([0, 1]);
    expect(iter.next().value).toEqual([2, 3]);
    expect(iter.next().value).toEqual([4, undefined]);
  });
});
