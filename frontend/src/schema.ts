/**
 * This file was auto-generated by openapi-typescript.
 * Do not make direct changes to the file.
 */


export interface paths {
  "/follow": {
    post: operations["create_follow"];
    delete: operations["delete_follow"];
  };
  "/followees/{name}": {
    get: operations["get_followees_by_user_name"];
  };
  "/followers/{name}": {
    get: operations["get_followers_by_user_name"];
  };
  "/login": {
    post: operations["login"];
  };
  "/me": {
    get: operations["me"];
    post: operations["update_me"];
  };
  "/posts": {
    get: operations["get_posts_by_user_name"];
    post: operations["create_post"];
  };
  "/signup": {
    post: operations["signup"];
  };
  "/users/{name}": {
    get: operations["user_profile"];
  };
}

export type webhooks = Record<string, never>;

export interface components {
  schemas: {
    readonly ErrorMessage: {
      readonly error: string;
    };
    readonly Follow: {
      readonly follow_from_name: string;
      readonly follow_to_name: string;
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
      readonly user_name: string;
    };
    readonly SignupCredential: {
      /** @example alice */
      readonly name: string;
      /** @example password */
      readonly password: string;
    };
    readonly UserProfile: {
      readonly avatar_url: string;
      readonly description: string;
      readonly display_name: string;
      readonly name: string;
    };
    readonly UserProfileUpdate: {
      readonly avatar_url: string;
      readonly description: string;
      readonly display_name: string;
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

  create_follow: {
    readonly requestBody: {
      readonly content: {
        readonly "application/json": components["schemas"]["Follow"];
      };
    };
    responses: {
      /** @description Successfully follow the user */
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
  delete_follow: {
    readonly requestBody: {
      readonly content: {
        readonly "application/json": components["schemas"]["Follow"];
      };
    };
    responses: {
      /** @description Successfully remove the user */
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
  get_followees_by_user_name: {
    parameters: {
      readonly path: {
        name: string;
      };
    };
    responses: {
      /** @description Successfully get followees for a user */
      200: {
        content: {
          readonly "application/json": readonly (components["schemas"]["UserProfile"])[];
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
  get_followers_by_user_name: {
    parameters: {
      readonly path: {
        name: string;
      };
    };
    responses: {
      /** @description Successfully get followers for a user */
      200: {
        content: {
          readonly "application/json": readonly (components["schemas"]["UserProfile"])[];
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
          readonly "application/json": components["schemas"]["UserProfile"];
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
  update_me: {
    responses: {
      /** @description Successfully update user profile */
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
  get_posts_by_user_name: {
    responses: {
      /** @description Successfully get posts for a user */
      200: {
        content: {
          readonly "application/json": readonly (components["schemas"]["Post"])[];
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
  user_profile: {
    parameters: {
      readonly path: {
        name: string;
      };
    };
    responses: {
      /** @description Successfully fetched user info */
      200: {
        content: {
          readonly "application/json": components["schemas"]["UserProfile"];
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
