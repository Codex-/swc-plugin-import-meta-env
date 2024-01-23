describe("typescript", () => {
  it("should make import.meta.env use process.env and expose the object", () => {
    expect(typeof import.meta.env).toStrictEqual("object");
  });
});
