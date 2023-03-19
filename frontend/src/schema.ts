/**
 * This file was auto-generated by openapi-typescript.
 * Do not make direct changes to the file.
 */

export interface paths {
    "/": {
        get: operations["home"];
    };
    "/login": {
        post: operations["login"];
    };
    "/signup": {
        post: operations["signup"];
    };
    "/users/{name}": {
        get: operations["user_info"];
    };
}

export type webhooks = Record<string, never>;

export interface components {
    schemas: {
        ErrorMessage: {
            error: string;
        };
        LoginCredential: {
            name: string;
            password: string;
        };
        SignupCredential: {
            /** @example alice */
            name: string;
            /** @example password */
            password: string;
        };
        UserInfoResponse: {
            /** @example alice */
            name: string;
        };
    };
    responses: never;
    parameters: never;
    requestBodies: never;
    headers: never;
    pathItems: never;
}

export type external = Record<string, never>;

export interface operations {
    home: {
        responses: {
            /** @description Successfully fetched user info */
            200: {
                content: {
                    "application/json": components["schemas"]["UserInfoResponse"];
                };
            };
            /** @description Unauthorized */
            401: {
                content: {
                    "application/json": components["schemas"]["ErrorMessage"];
                };
            };
            /** @description InternalServerError */
            500: {
                content: {
                    "application/json": components["schemas"]["ErrorMessage"];
                };
            };
        };
    };
    login: {
        requestBody: {
            content: {
                "application/json": components["schemas"]["LoginCredential"];
            };
        };
        responses: {
            /** @description Successfully logged in */
            200: never;
            /** @description Unauthorized */
            401: {
                content: {
                    "application/json": components["schemas"]["ErrorMessage"];
                };
            };
            /** @description InternalServerError */
            500: {
                content: {
                    "application/json": components["schemas"]["ErrorMessage"];
                };
            };
        };
    };
    signup: {
        requestBody: {
            content: {
                "application/json": components["schemas"]["SignupCredential"];
            };
        };
        responses: {
            /** @description Successfully created a new user */
            200: never;
            /** @description InternalServerError */
            500: {
                content: {
                    "application/json": components["schemas"]["ErrorMessage"];
                };
            };
        };
    };
    user_info: {
        parameters: {
            path: {
                name: string;
            };
        };
        responses: {
            /** @description Successfully fetched user info */
            200: {
                content: {
                    "application/json": components["schemas"]["UserInfoResponse"];
                };
            };
            /** @description BadRequest */
            400: {
                content: {
                    "application/json": components["schemas"]["ErrorMessage"];
                };
            };
            /** @description InternalServerError */
            500: {
                content: {
                    "application/json": components["schemas"]["ErrorMessage"];
                };
            };
        };
    };
}
