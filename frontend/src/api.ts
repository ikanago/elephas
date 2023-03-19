import { type components, type operations } from "./schema";

const api = import.meta.env.DEV
    ? "http://localhost:5173/api"
    : "http://localhost:3000/api";

type ErrorMessage = components["schemas"]["ErrorMessage"];

export const home = async () => {
    const res = await fetch(`${api}/`, {
        method: "GET",
        credentials: "include",
    });

    if (!res.ok) {
        const json: ErrorMessage = await res.json();
        throw new Error(json.error);
    }
    return await res.json() as components["schemas"]["UserInfoResponse"];
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
        const json: ErrorMessage = await res.json();
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
        const json: ErrorMessage = await res.json();
        throw new Error(json.error);
    }
};

export const user_info = async (
    name: operations["user_info"]["parameters"]["path"]["name"]
) => {
    const res = await fetch(`${api}/user_info/${name}`, {
        method: "GET",
        credentials: "include",
    });

    if (!res.ok) {
        const json: ErrorMessage = await res.json();
        throw new Error(json.error);
    }
    return await res.json() as components["schemas"]["UserInfoResponse"];
};
