import { components, type operations } from "./schema";

const api = import.meta.env.DEV
    ? "http://localhost:5173/api"
    : "http://localhost:3000/api";

type ErrorMessage = components["schemas"]["ErrorMessage"];

export const home = async () => {
    return await fetch(`${api}/`, {
        method: "GET",
        credentials: "include",
    });
};

export const signup = async (
    payload: operations["signup"]["requestBody"]["content"]["application/json"]
) => {
    const res = await fetch(`${api}/signup`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
    });

    if (!res.ok) {
        const json = await res.json() as ErrorMessage;
        throw new Error(json.error);
    }
};

export const login = async (
    payload: operations["login"]["requestBody"]["content"]["application/json"]
) => {
    const res = await fetch(`${api}/login`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
    });

    if (!res.ok) {
        const json = await res.json() as ErrorMessage;
        throw new Error(json.error);
    }
};
