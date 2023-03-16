const api = import.meta.env.DEV ? "http://localhost:5173/api" : "http://localhost:3000/api";

export const home = async () => {
    return await fetch(`${api}/`, {
        method: "GET",
        credentials: "include",
    });
};

export const signup = async (name: string, password: string) => {
    return await fetch(`${api}/signup`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            name,
            password,
        }),
    });
};

export const login = async (name: string, password: string) => {
    return await fetch(`${api}/login`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            name,
            password,
        }),
    });
};
