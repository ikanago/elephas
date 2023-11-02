import { type paths } from "./schema";

const api =
  import.meta.env.MODE === "test"
    ? "http://localhost:3000/api"
    : import.meta.env.MODE === "development"
    ? "http://localhost:5173/api"
    : "https://elephas-dev.ikanago.dev/api";

type ResponseOperation<Op> = Op extends { responses: infer Statuses }
  ? {
      [K in keyof Statuses]: Statuses[K] extends { content: infer Content }
        ? {
            status: K;
            data: Content extends { "application/json": infer Data }
              ? Data
              : never;
          }
        : never;
    }[keyof Statuses]
  : never;

export type ResponseGet<P extends keyof paths> = paths[P] extends {
  get: infer Get;
}
  ? ResponseOperation<Get>
  : never;

export type ResponsePost<P extends keyof paths> = paths[P] extends {
  post: infer Post;
}
  ? ResponseOperation<Post>
  : never;

export type ResponseDelete<P extends keyof paths> = paths[P] extends {
  delete: infer Delete;
}
  ? ResponseOperation<Delete>
  : never;

type Parameters<P extends keyof paths> = paths[P] extends { get: infer Get }
  ? Get extends { parameters: infer Params }
    ? Params extends { path: infer Path }
      ? Path
      : never
    : never
  : never;

type Payload<P extends keyof paths> = paths[P] extends { post: infer Post }
  ? Post extends {
      requestBody: {
        content: {
          "application/json": infer Body;
        };
      };
    }
    ? Body
    : undefined
  : undefined;

const apiGet =
  <P extends keyof paths>(path: P) =>
  async (params?: Parameters<P>): Promise<ResponseGet<P>> => {
    let replacedPath;
    if (params !== undefined) {
      replacedPath = path.replace(/{([^}]*)}/g, (_, key) => params[key]);
    } else {
      replacedPath = path;
    }
    const res = await fetch(`${api}${replacedPath}`, {
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

const apiPost =
  <P extends keyof paths>(path: P) =>
  async (payload: Payload<P>): Promise<ResponsePost<P>> => {
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

const apiDelete =
  <P extends keyof paths>(path: P) =>
  async (payload: Payload<P>): Promise<ResponseDelete<P>> => {
    const res = await fetch(`${api}${path}`, {
      method: "DELETE",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });

    if (res.status === 204) {
      // eslint-disable-next-line @typescript-eslint/consistent-type-assertions
      return {
        status: res.status,
      } as ResponseDelete<P>;
    }

    const data = await res.json();
    return {
      status: res.status,
      data,
    } as unknown as ResponseDelete<P>;
  };

export const me = apiGet("/me");

export const updateMe = apiPost("/me");

export const signup = apiPost("/signup");

export const login = apiPost("/login");

export const createPost = apiPost("/posts");

export const getPostsOfMe = apiGet("/posts");

export const getUserProfile = apiGet("/users/{user_name}");

export const createFollow = apiPost("/follow");

export const deleteFollow = apiDelete("/follow");

export const getFollowees = apiGet("/followees/{user_name}");

export const getFollowers = apiGet("/followers/{user_name}");
