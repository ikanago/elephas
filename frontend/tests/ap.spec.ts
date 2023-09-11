import { expect, test } from "@playwright/test";
import { baseURL, resetDb, signupUser } from "./util";

const replaceUrlSchemeWithHttps = (url: string) => {
    return url.replace("http://", "https://");
};

test.afterAll(async () => {
  await resetDb();
});

test.describe("webfinger", () => {
  const username = "cat";
  const password = "pass";

  test.beforeAll(async () => {
    await resetDb();
    await signupUser(username, password);
  });

  test("get successfully", async ({ request }) => {
    const webfinger = await request.get(
      `${baseURL}/.well-known/webfinger?resource=acct:${username}@${baseURL}`,
    );
    const json = await webfinger.json();

    expect(webfinger.status()).toBe(200);
    // Workaround: API always returns URL starting with https://
    expect(json["links"][0]["href"]).toBe(replaceUrlSchemeWithHttps(`${baseURL}/api/users/${username}`));
  });
});
