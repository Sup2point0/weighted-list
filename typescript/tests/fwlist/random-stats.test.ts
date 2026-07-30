import { describe, test, assert } from "vitest";

import { test_binomial } from "../stats";
import { fwl } from "./shared";

import { FWList } from "../../src";


describe.for(
  ["sample-value", "sample-values"]
)("sample stats", method =>
{
  describe("integer weights", () =>
  {
    test("equal probabilities", () => {
      test_binomial(method, new FWList(
        [1, "left"],
        [1, "right"],
      ));
    });
    
    test("usual probabilities", () => {
      test_binomial(method, fwl());
    });
    
    test("zero probabilities", () => {
      test_binomial(method, new FWList(
        [0, "no"],
        [1, "yes"],
      ));
    });
    
    test("extreme probabilities", () => {
      test_binomial(method, new FWList(
        [100, "sup"],
        [1, "ayo"],
      ));
      
      test_binomial(method, new FWList(
        [1000, "sup"],
        [1, "ayo"],
      ));
    });
  });
});
