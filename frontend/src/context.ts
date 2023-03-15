import { createContext, useContext } from "react";

interface Context {
    isAuthenticated: boolean;
    authenticate: (callback: VoidFunction) => void;
    unauthenticate: (callback: VoidFunction) => void;
}

export const AuthContext = createContext<Context>({
    isAuthenticated: false,
    authenticate: (callback: VoidFunction) => {
        callback();
    },
    unauthenticate: (callback: VoidFunction) => {
        callback();
    },
});

export const useAuth = () => useContext(AuthContext);
