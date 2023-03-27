/**
 * This file was auto-generated by openapi-typescript.
 * Do not make direct changes to the file.
 */


export interface paths {
  "/login": {
    post: operations["login"];
  };
  "/me": {
    get: operations["me"];
  };
  "/posts": {
    get: operations["get_posts_by_user_id"];
    post: operations["create_post"];
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
    readonly ErrorMessage: {
      readonly error: string;
    };
    readonly LoginCredential: {
      readonly name: string;
      readonly password: string;
    };
    readonly NewPost: {
      readonly content: string;
    };
    readonly Post: {
      readonly content: string;
      readonly id: string;
      /** Format: date-time */
      readonly published_at: string;
      readonly user_id: string;
    };
    readonly SignupCredential: {
      /** @example alice */
      readonly name: string;
      /** @example password */
      readonly password: string;
    };
    readonly UserInfoResponse: {
      /** @example alice */
      readonly name: string;
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

  login: {
    readonly requestBody: {
      readonly content: {
        readonly "application/json": components["schemas"]["LoginCredential"];
      };
    };
    responses: {
      /** @description Successfully logged in */
      204: never;
      /** @description Unauthorized */
      401: {
        content: {
          readonly "application/json": components["schemas"]["ErrorMessage"];
        };
      };
      /** @description InternalServerError */
      500: {
        content: {
          readonly "application/json": components["schemas"]["ErrorMessage"];
        };
      };
    };
  };
  me: {
    responses: {
      /** @description Successfully fetched user info */
      200: {
        content: {
          readonly "application/json": components["schemas"]["UserInfoResponse"];
        };
      };
      /** @description Unauthorized */
      401: {
        content: {
          readonly "application/json": components["schemas"]["ErrorMessage"];
        };
      };
      /** @description InternalServerError */
      500: {
        content: {
          readonly "application/json": components["schemas"]["ErrorMessage"];
        };
      };
    };
  };
  get_posts_by_user_id: {
    responses: {
      /** @description Successfully get posts for a user */
      200: {
        content: {
          readonly "application/json": ReadonlyArray<components["schemas"]["Post"]>;
        };
      };
      /** @description Unauthorized */
      401: {
        content: {
          readonly "application/json": components["schemas"]["ErrorMessage"];
        };
      };
      /** @description InternalServerError */
      500: {
        content: {
          readonly "application/json": components["schemas"]["ErrorMessage"];
        };
      };
    };
  };
  create_post: {
    readonly requestBody: {
      readonly content: {
        readonly "application/json": components["schemas"]["NewPost"];
      };
    };
    responses: {
      /** @description Successfully created a post */
      204: never;
      /** @description Unauthorized */
      401: {
        content: {
          readonly "application/json": components["schemas"]["ErrorMessage"];
        };
      };
      /** @description InternalServerError */
      500: {
        content: {
          readonly "application/json": components["schemas"]["ErrorMessage"];
        };
      };
    };
  };
  signup: {
    readonly requestBody: {
      readonly content: {
        readonly "application/json": components["schemas"]["SignupCredential"];
      };
    };
    responses: {
      /** @description Successfully created a new user */
      204: never;
      /** @description InternalServerError */
      500: {
        content: {
          readonly "application/json": components["schemas"]["ErrorMessage"];
        };
      };
    };
  };
  user_info: {
    parameters: {
      readonly path: {
        name: string;
      };
    };
    responses: {
      /** @description Successfully fetched user info */
      200: {
        content: {
          readonly "application/json": components["schemas"]["UserInfoResponse"];
        };
      };
      /** @description BadRequest */
      400: {
        content: {
          readonly "application/json": components["schemas"]["ErrorMessage"];
        };
      };
      /** @description InternalServerError */
      500: {
        content: {
          readonly "application/json": components["schemas"]["ErrorMessage"];
        };
      };
    };
  };
}
