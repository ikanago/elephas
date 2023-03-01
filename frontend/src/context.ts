import React, {createContext, useContext} from "react";

type Context = {
    isAuthenticated: boolean;
    authenticate: (callback: VoidFunction) => void;
    unauthenticate: (callback: VoidFunction) => void;
}

export const AuthContext = createContext<Context>({
    isAuthenticated: false,
    authenticate: (callback: VoidFunction) => {},
    unauthenticate: (callback: VoidFunction) => {},
});

export const useAuth = () => useContext(AuthContext);
