import { describe, test, assert } from "vitest";

import { test_binomial_for } from "../stats";
import { fwl } from "./shared";

import { FWList } from "../../src";


describe.for(
  ["sample-value", "sample-values"]
)("sample stats", method =>
{
  describe("integer weights", () =>
  {
    test("equal probabilities", () => {
      test_binomial_for(method, new FWList(
        [1, "left"],
        [1, "right"],
      ));
    });
    
    test("usual probabilities", () => {
      test_binomial_for(method, fwl());
    });
    
    test("zero probabilities", () => {
      test_binomial_for(method, new FWList(
        [1, "yes"],
        [0, "no"],
      ));
    });
    
    test("extreme probabilities", () => {
      test_binomial_for(method, new FWList(
        [100, "sup"],
        [1, "ayo"],
      ));
      
      test_binomial_for(method, new FWList(
        [1000, "sup"],
        [1, "ayo"],
      ));
    });
  });

  describe("fractional weights", () =>
  {
    test("equal probabilities", () => {
      test_binomial_for(method, new FWList(
        [0.5, "left"],
        [0.5, "right"],
      ));
    });
    
    // test("usual probabilities", () => {
    //   test_binomial(method, fwl().normalised());
    // });
    
    test("zero probabilities", () => {
      test_binomial_for(method, new FWList(
        [0.1, "yes"],
        [0, "no"],
      ));
    });
    
    test("extreme probabilities", () => {
      test_binomial_for(method, new FWList(
        [10, "sup"],
        [0.1, "ayo"],
      ));
      
      test_binomial_for(method, new FWList(
        [100, "sup"],
        [0.1, "ayo"],
      ));
    });
  });
});
