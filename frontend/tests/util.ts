import config from "../playwright.config";

export const baseURL = config.use?.baseURL!;

export const resetDb = async () => {
  await fetch(`${baseURL}/api/reset-db`, {
    method: "DELETE",
  });
};

export const signupUser = async (username: string, password: string) => {
  await fetch(`${baseURL}/api/signup`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ name: username, password: password }),
  });
};
