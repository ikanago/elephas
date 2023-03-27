import { type paths } from "./schema";

const api = import.meta.env.DEV
    ? "http://localhost:5173/api"
    : "http://localhost:3000/api";

export type ResponseGet<P extends keyof paths> = paths[P] extends { get: infer Get }
    ? Get extends { responses: infer Statuses }
        ? {
            [K in keyof Statuses]:
                Statuses[K] extends { content: infer Content }
                    ? Content extends { "application/json": infer Data }
                        ? { status: K, data: Data }
                        : { status: K }
                    : never;
        }[keyof Statuses]
        : never
    : never; 

type Payload<P extends keyof paths> = paths[P] extends { post: infer Post }
    ? Post extends 
        {
            requestBody: {
                content: {
                    "application/json": infer Body
                }
            }
        }
        ? Body : undefined
    : undefined;

export type ResponsePost<P extends keyof paths> = paths[P] extends { post: infer Post }
    ? Post extends { responses: infer Statuses }
        ? {
            [K in keyof Statuses]:
                Statuses[K] extends { content: infer Content }
                    ? Content extends { "application/json": infer Response }
                        ? { status: K, data: Response }
                        : { status: K }
                    : never;
        }[keyof Statuses]
        : never
    : never; 

const get = <P extends keyof paths>(path: P) => async (): Promise<ResponseGet<P>> => {
    const res = await fetch(`${api}${path}`, {
        method: "GET",
        credentials: "include",
    });

    const data = await res.json();
    // eslint-disable-next-line @typescript-eslint/consistent-type-assertions
    const response = {
        status: res.status,
        data,
    } as ResponseGet<P>;
    return response;
};

const post = <P extends keyof paths>(path: P) => async (payload: Payload<P>): Promise<ResponsePost<P>> => {
    const res = await fetch(`${api}${path}`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
    });

    if (res.status === 204) {
        // eslint-disable-next-line @typescript-eslint/consistent-type-assertions
        return {
            status: res.status,
        } as ResponsePost<P>;
    }

    const data = await res.json();
    return {
        status: res.status,
        data,
    } as unknown as ResponsePost<P>;
};

export const me = get("/me");

export const signup = post("/signup");

export const login = post("/login");

export const createPost = post("/posts");

export const getPostsOfMe = get("/posts");
