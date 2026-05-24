import { render, screen } from "@testing-library/react";
import { App } from "./App";

describe("App", () => {
  it("renders the M0 foundation shell", () => {
    render(<App />);

    expect(screen.getByRole("heading", { name: "VipaVault" })).toBeInTheDocument();
    expect(screen.getByText("M0 Foundation")).toBeInTheDocument();
  });
});
