import { describe, it } from "mocha";

describe("sign up", () => {
    it("passes", () => {
        // arrange
        cy.intercept("POST", "/api/signup", {
            statusCode: 200,
        }).as("signup");

        // act
        cy.visit("/signup");
        cy.get('input[name="username"]').type("cat");
        cy.get('input[name="password"]').type("pass");
        cy.get('input[type="submit"]').click();
        cy.wait("@signup");

        // assert
        cy.location("pathname").should("eq", "/");
        // TODO: assert that COOKIE exists
    });
});
