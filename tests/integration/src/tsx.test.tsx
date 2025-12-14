import React from "react";
import { render, screen } from "@testing-library/react";

interface ImportMeta {
  readonly env: Record<string, any>;
}

const TestComponent: React.FC = () => {
  const undefinedValue = typeof import.meta.env.attempt;

  return <div data-testid="app">{undefinedValue}</div>;
};

describe("tsx", () => {
  it("should make import.meta.env use process.env and expose the object", () => {
    expect(typeof import.meta.env).toStrictEqual("object");
  });

  it("should render a component using import.meta.env", () => {
    render(<TestComponent />);
    const element = screen.getByTestId("app");
    expect(element).not.toBe(true);
  });
});
