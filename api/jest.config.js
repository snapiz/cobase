module.exports = {
  roots: ["<rootDir>/src"],
  transform: {
    "^.+\\.ts$": "ts-jest",
  },
  preset: "ts-jest",
  //testRegex: "(/__tests__/.*|(\\.|/)(test|spec))\\.ts$",
  testRegex: "/__tests__/.*\\.spec\\.ts$",
  moduleFileExtensions: ["ts", "js", "json", "node"],
};
