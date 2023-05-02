import { expect, test } from "vitest";



test("first test", () => {
    expect(1 + 1).toBe(2)
})


// test("invoke simple", async () => {
//   mockIPC((cmd, args) => {
//     // simulated rust command called "add" that just adds two numbers
//     if(cmd === "add") {
//       return (args.a as number) + (args.b as number);
//     }
//   });
// });