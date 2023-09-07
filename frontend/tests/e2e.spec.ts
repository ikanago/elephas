import { expect, test } from "@playwright/test";
import config from "../playwright.config";

const baseURL = config.use!.baseURL!;

const resetDb = async () => {
    await fetch(`${baseURL}/api/reset-db`, {
        method: "DELETE",
    });
}

const signupUser = async (username: string, password: string) => {
    await fetch(`${baseURL}/api/signup`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ name: username, password: password }),
    });
};

test.afterAll(async () => {
  resetDb();
});

test.describe("sign up", () => {
  test.beforeEach(async () => {
    resetDb();
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
    await expect(page.locator(".error")).toHaveText("The user name is already used");
  });
});
