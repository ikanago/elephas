import { expect, test } from "@playwright/test";
import { baseURL, resetDb, signupUser } from "./util";

test.afterAll(async () => {
  await resetDb();
});

test.describe("sign up", () => {
  test.beforeEach(async () => {
    await resetDb();
  });

  test("passes", async ({ page }) => {
    // act
    await page.goto("/signup");
    await page.fill('input[name="username"]', "cat");
    await page.fill('input[name="password"]', "pass");
    await page.click('input[type="submit"]');
    await page.waitForURL(baseURL);

    // assert
    const cookies = await page.context().cookies();
    expect(cookies).toHaveLength(1);
    expect(cookies[0].name).toBe("id");
    await expect(page.locator("h1")).toHaveText("cat");
  });

  test("w/ already signed up user name fails", async ({ page }) => {
    const username = "cat";
    const password = "pass";

    // act
    await signupUser(username, password);

    // sign up again
    await page.goto("/signup");
    await page.fill('input[name="username"]', username);
    await page.fill('input[name="password"]', password);
    await page.click('input[type="submit"]');

    // assert
    await expect(page).toHaveURL("/signup");
    await expect(page.locator(".error")).toHaveText(
      "The user name is already used",
    );
  });
});

test.describe("log in", () => {
  const username = "cat";
  const password = "pass";

  // Log in test does not register a new user, so we need to do it before test just once.
  test.beforeAll(async () => {
    await resetDb();
    await signupUser(username, password);
  });

  test("passes", async ({ page }) => {
    // act
    await page.goto("/login");
    await page.fill('input[name="username"]', username);
    await page.fill('input[name="password"]', password);
    await page.click('input[type="submit"]');
    await page.waitForURL(baseURL);

    // assert
    const cookies = await page.context().cookies();
    expect(cookies).toHaveLength(1);
    expect(cookies[0].name).toBe("id");
    await expect(page.locator("h1")).toHaveText("cat");
  });

  test("fails with wrong password", async ({ page }) => {
    // act
    await page.goto("/login");
    await page.fill('input[name="username"]', username);
    await page.fill('input[name="password"]', "wrong");
    await page.click('input[type="submit"]');

    // assert
    await expect(page).toHaveURL("/login");
    expect(await page.context().cookies()).toHaveLength(0);
    await expect(page.locator(".error")).toHaveText(
      "User name or password is wrong",
    );
  });

  test("fails with wrong user name", async ({ page }) => {
    // act
    await page.goto("/login");
    await page.fill('input[name="username"]', "wrong");
    await page.fill('input[name="password"]', password);
    await page.click('input[type="submit"]');

    // assert
    await expect(page).toHaveURL("/login");
    expect(await page.context().cookies()).toHaveLength(0);
    await expect(page.locator(".error")).toHaveText("The user is not found");
  });
});

test.describe("post", () => {
  const username = "cat";
  const password = "pass";

  test.beforeAll(async () => {
    await resetDb();
    await signupUser(username, password);
  });

  test("create", async ({ page }) => {
    // arrange
    await page.goto("/login");
    await page.fill('input[name="username"]', username);
    await page.fill('input[name="password"]', password);
    await page.click('input[type="submit"]');
    await page.waitForURL(baseURL);

    // act
    await page.goto("/");
    await page.fill('input[name="content"]', "hello");
    await page.click('input[type="submit"]');

    // assert
    await expect(page).toHaveURL("/");
    await expect(page.locator(".timeline")).toHaveCount(1);
    await expect(page.locator(".timeline").locator(".post")).toHaveText(
      "hello",
    );
  });
});

test.describe("user profile", () => {
  const username = "cat";
  const password = "pass";

  test.beforeAll(async () => {
    await resetDb();
    await signupUser(username, password);
  });

  test.beforeEach(async ({ page }) => {
    await page.goto("/login");
    await page.fill('input[name="username"]', username);
    await page.fill('input[name="password"]', password);
    await page.click('input[type="submit"]');
    await page.waitForURL(baseURL);
  });

  test("get successfully", async ({ page }) => {
    // act
    await page.goto("/users/cat");

    // assert
    await expect(page).toHaveURL("/users/cat");
    await expect(page.locator(".name")).toHaveText("cat");
  });

  test("not found for non-existing user", async ({ page }) => {
    // act
    await page.goto("/users/caaaaat");

    // assert
    await expect(page).toHaveURL("/users/caaaaat");
    await expect(page.locator("p")).toHaveText("Not found");
  });

  test("edit successfully", async ({ page }) => {
    // act
    await page.goto("/users/cat");
    await page.click(".edit");

    // assert
    await expect(page).toHaveURL("/settings/profile");

    // act
    const displayName = "meow";
    const summary = "I'm a cat";
    const avatarUrl = "https://placekitten.com/200/300";
    await page.fill('input[name="displayName"]', displayName);
    await page.fill('input[name="summary"]', summary);
    await page.fill('input[name="avatarUrl"]', avatarUrl);
    await page.click('input[type="submit"]');

    // assert
    await expect(page).toHaveURL("/");

    // act
    await page.goto("/users/cat");

    // assert
    await expect(page.locator(".displayName")).toHaveText(displayName);
    await expect(page.locator(".summary")).toHaveText(summary);
    await expect(page.locator(".avatarUrl")).toHaveText(avatarUrl);
  });

  test("failes editing by users other than the logged in user", async ({
    page,
  }) => {
    // act
    await page.goto("/users/dog");

    // assert
    await expect(page).toHaveURL("/users/dog");
    await expect(page.locator(".edit")).not.toBeVisible();
  });
});

test.describe("follow", () => {
  test.beforeAll(async () => {
    await resetDb();
    await signupUser("dog", "pass");
    await signupUser("cat", "pass");
  });

  test("successfully", async ({ page }) => {
    // arrange
    await page.goto("/login");
    await page.fill('input[name="username"]', "cat");
    await page.fill('input[name="password"]', "pass");
    await page.click('input[type="submit"]');
    await page.waitForURL(baseURL);

    // act
    await page.goto("/users/dog");
    await page.click(".follow");
    await page.waitForURL("/users/dog");

    // assert
    await expect(page.locator(".followees")).toHaveText("0 follows");
    await expect(page.locator(".followers")).toHaveText("1 followers");
    await expect(page.locator(".unfollow")).toHaveText("Unfollow");

    await page.goto("/users/cat");
    await expect(page.locator(".followees")).toHaveText("1 follows");
    await expect(page.locator(".followers")).toHaveText("0 followers");

    // act
    await page.goto("/users/dog");
    await page.click(".unfollow");
    await page.waitForURL("/users/dog");

    // assert
    await expect(page.locator(".followees")).toHaveText("0 follows");
    await expect(page.locator(".followers")).toHaveText("0 followers");
    await expect(page.locator(".follow")).toHaveText("Follow");

    await page.goto("/users/cat");
    await expect(page.locator(".followees")).toHaveText("0 follows");
    await expect(page.locator(".followers")).toHaveText("0 followers");
  });
});
