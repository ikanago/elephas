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

    it("w/ already signed up user name fails", () => {
        // arrange
        cy.intercept("POST", "/api/signup").as("signup");

        // act
        const username = "cat";
        const password = "pass";
        cy.visit("/signup");
        cy.get('input[name="username"]').type(username);
        cy.get('input[name="password"]').type(password);
        cy.get('input[type="submit"]').click();
        cy.wait("@signup");

        // sign up again
        cy.visit("/signup");
        cy.get('input[name="username"]').type(username);
        cy.get('input[name="password"]').type(password);
        cy.get('input[type="submit"]').click();
        cy.wait("@signup");

        // assert
        cy.location("pathname").should("eq", "/signup");
        cy.get(".error").should("have.text", "The user name is already used.");
    });
});

describe("log in", () => {
    const username = "cat";
    const password = "pass";

    // Log in test does not register a new user, so we need to do it before test just once.
    before(() => {
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
        cy.get("h1").should("have.text", "cat");
    });

    it("failes with wrong password", () => {
        // arrange
        cy.intercept("POST", "/api/login").as("login");

        // act
        cy.visit("/login");
        cy.get('input[name="username"]').type(username);
        cy.get('input[name="password"]').type("wrong password");
        cy.get('input[type="submit"]').click();
        cy.wait("@login");

        // assert
        cy.location("pathname").should("eq", "/login");
        cy.getAllCookies().should("have.length", 0);
        cy.get(".error").should("have.text", "User name or password is wrong.");
    });
});
