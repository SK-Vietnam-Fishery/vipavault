import { render, screen } from "@testing-library/react";
import { App } from "./App";

describe("App", () => {
  it("renders the 0.1.0 foundation shell", () => {
    render(<App />);

    expect(screen.getByRole("heading", { name: "VipaVault" })).toBeInTheDocument();
    expect(screen.getByText("0.1.0 Foundation")).toBeInTheDocument();
  });

  it("shows package version in boot health panel", () => {
    render(<App />);

    expect(screen.getByTestId("app-version")).toHaveTextContent("v0.1.0");
  });
});
