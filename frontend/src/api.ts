import { Result } from "ts-results";
import { type components, type operations } from "./schema";

const api = import.meta.env.DEV
    ? "http://localhost:5173/api"
    : "http://localhost:3000/api";

type ErrorMessage = components["schemas"]["ErrorMessage"];

export const me = async () => {
    return await Result.wrapAsync<
        components["schemas"]["UserInfoResponse"],
        ErrorMessage
    >(async () => {
        const res = await fetch(`${api}/me`, {
            method: "GET",
            credentials: "include",
        });

        if (!res.ok) {
            const json = await res.json();
            throw json;
        }
        return await res.json();
    });
};

export const signup = async (
    payload: operations["signup"]["requestBody"]["content"]["application/json"]
) => {
    return await Result.wrapAsync<undefined, ErrorMessage>(async () => {
        const res = await fetch(`${api}/signup`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(payload),
        });

        if (!res.ok) {
            const json: ErrorMessage = await res.json();
            throw json;
        }

        // Workaround for eslint warning: @typescript-eslint/no-invalid-void-type complains Result.wrapAsync's void return type.
        return undefined;
    });
};

export const login = async (
    payload: operations["login"]["requestBody"]["content"]["application/json"]
) => {
    return await Result.wrapAsync<undefined, ErrorMessage>(async () => {
        const res = await fetch(`${api}/login`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(payload),
        });

        if (!res.ok) {
            const json: ErrorMessage = await res.json();
            throw json;
        }

        return undefined;
    });
};

export const createPost = async (
    payload: operations["create_post"]["requestBody"]["content"]["application/json"]
) => {
    return await Result.wrapAsync<undefined, ErrorMessage>(async () => {
        const res = await fetch(`${api}/posts`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(payload),
        });

        if (!res.ok) {
            const json: ErrorMessage = await res.json();
            throw json;
        }

        return undefined;
    });
};

export const getPostsOfMe = async () => {
    return await Result.wrapAsync<
        Array<components["schemas"]["Post"]>,
        ErrorMessage
    >(async () => {
        const res = await fetch(`${api}/posts`, {
            method: "GET",
            credentials: "include",
        });

        if (!res.ok) {
            const json: ErrorMessage = await res.json();
            throw json;
        }

        return await res.json();
    });
};
