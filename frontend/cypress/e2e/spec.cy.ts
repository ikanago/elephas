import { describe, it } from "mocha";

describe("sign up", () => {
    it("passes", () => {
        // arrange
        cy.intercept("POST", "/api/signup").as("signup");

        // act
        cy.visit("/signup");
        cy.get('input[name="username"]').type("cat");
        cy.get('input[name="password"]').type("pass");
        cy.get('input[type="submit"]').click();
        cy.wait("@signup");

        // assert
        cy.location("pathname").should("eq", "/");
        cy.getAllCookies()
            .should("have.length", 1)
            .then(cookies => {
                expect(cookies[0].name).to.eq("id");
            })
    });
});
