import { describe, it } from "mocha";

describe("sign up", () => {
    beforeEach(() => {
        cy.resetState();
    });

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
            });
    });
});

describe("log in", () => {
    const username = "cat";
    const password = "pass";

    beforeEach(() => {
        cy.resetState();
        cy.signupUser(username, password);
    });

    it("passes", () => {
        // arrange
        cy.intercept("POST", "/api/login").as("login");

        // act
        cy.visit("/login");
        cy.get('input[name="username"]').type(username);
        cy.get('input[name="password"]').type(password);
        cy.get('input[type="submit"]').click();
        cy.wait("@login");

        // assert
        cy.location("pathname").should("eq", "/");
        cy.getAllCookies()
            .should("have.length", 1)
            .then(cookies => {
                expect(cookies[0].name).to.eq("id");
            });
    });
});
