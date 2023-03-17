// ***********************************************
// This example commands.ts shows you how to
// create various custom commands and overwrite
// existing commands.
//
// For more comprehensive examples of custom
// commands please read more here:
// https://on.cypress.io/custom-commands
// ***********************************************
//
//
// -- This is a parent command --
// Cypress.Commands.add('login', (email, password) => { ... })
//
//
// -- This is a child command --
// Cypress.Commands.add('drag', { prevSubject: 'element'}, (subject, options) => { ... })
//
//
// -- This is a dual command --
// Cypress.Commands.add('dismiss', { prevSubject: 'optional'}, (subject, options) => { ... })
//
//
// -- This will overwrite an existing command --
// Cypress.Commands.overwrite('visit', (originalFn, url, options) => { ... })
//
declare global {
    namespace Cypress {
        interface Chainable {
            resetState(): Chainable<void>
            signupUser(username: string, password: string): Chainable<void>
        }
    }
}

Cypress.Commands.add("resetState", () => {
    cy.clearAllCookies();
    cy.request("DELETE", "/api/reset-db").as("reset-db");
});

Cypress.Commands.add("signupUser", (username: string, password: string) => {
    cy.request("POST", "/api/signup", { name: username, password: password }).as("signup");
});

export {};
