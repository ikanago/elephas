import { after, describe, it } from "mocha";

after(() => {
    cy.resetState();
});

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
        cy.get(".error").should("have.text", "The user name is already used");
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
        cy.get(".error").should("have.text", "User name or password is wrong");
    });
});

describe("post", () => {
    before(() => {
        cy.resetState();
        cy.signupUser("cat", "pass");
    });

    it("create", () => {
        // arrange
        cy.intercept("POST", "/api/posts").as("createPost");
        cy.intercept("GET", "/api/posts").as("getPosts");

        // act
        cy.visit("/");
        cy.get('input[name="content"]').type("hello");
        cy.get('input[type="submit"]').click();
        cy.wait("@createPost");
        cy.wait("@getPosts");

        // assert
        cy.location("pathname").should("eq", "/");
        cy.get(".timeline").should("have.length", 1);
        cy.get(".timeline").get(".post").should("have.text", "hello");
    });

    // TODO: test there is an error when logged out
});

describe("user profile", () => {
    before(() => {
        cy.resetState();
        cy.signupUser("cat", "pass");
    });

    it("get successfully", () => {
        // arrange
        cy.intercept("GET", "/api/users/cat").as("getUserProfile");

        // act
        cy.visit("/users/cat");
        cy.wait("@getUserProfile");

        // assert
        cy.location("pathname").should("eq", "/users/cat");
        cy.get(".name").should("have.text", "cat");
    });

    it("not found for non-existing user", () => {
        // arrange
        cy.intercept("GET", "/api/users/caaaaat").as("getUserProfile");

        // act
        cy.visit("/users/caaaaat");
        cy.wait("@getUserProfile");

        // assert
        cy.location("pathname").should("eq", "/users/caaaaat");
        cy.get("p").should("have.text", "Not found");
    });
});

describe.only("follow", () => {
    beforeEach(() => {
        cy.resetState();
        cy.signupUser("dog", "pass");
        cy.clearAllCookies();
        cy.signupUser("cat", "pass");
    });

    it("successfully", () => {
        // arrange
        cy.intercept("POST", "/api/follow").as("follow");

        // act
        cy.visit("/users/dog");
        cy.get('button').click();
        cy.wait("@follow");

        // assert
        cy.location("pathname").should("eq", "/users/dog");
        cy.get(".followees").should("have.text", "0 follows");
        cy.get(".followers").should("have.text", "1 followers");
        cy.get("button").should("have.text", "Unfollow");

        cy.visit("/users/cat");
        cy.get(".followees").should("have.text", "1 follows");
        cy.get(".followers").should("have.text", "0 followers");
    });

    it("remove successfully", () => {
        // arrange
        cy.intercept("POST", "/api/follow").as("follow");
        cy.intercept("DELETE", "/api/follow").as("unfollow");

        // act
        cy.visit("/users/dog");
        cy.get('button').click();
        cy.wait("@follow");
        cy.get('button').click();
        cy.wait("@unfollow");

        // assert
        cy.location("pathname").should("eq", "/users/dog");
        cy.get(".followees").should("have.text", "0 follows");
        cy.get(".followers").should("have.text", "0 followers");
        cy.get("button").should("have.text", "Follow");

        cy.visit("/users/cat");
        cy.get(".followees").should("have.text", "0 follows");
        cy.get(".followers").should("have.text", "0 followers");
    });
});
