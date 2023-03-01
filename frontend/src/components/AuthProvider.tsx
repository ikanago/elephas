import { useEffect, useState } from "react";
import { AuthContext } from "../context";

const AuthProvider = ({ children }: { children: React.ReactNode }) => {
    const [isAuthenticated, setIsAuthenticated] = useState(false);

    const authenticate = (callback: VoidFunction) => {
        setIsAuthenticated(true);
        callback();
    }

    const unauthenticate = (callback: VoidFunction) => {
        setIsAuthenticated(true);
        callback();
    }

    useEffect(() => {
        (async () => {
            //
        })();
    }, []);

    return (
        <AuthContext.Provider value={{ isAuthenticated, authenticate, unauthenticate }}>
            {children}
        </AuthContext.Provider>
    )
};

export default AuthProvider;
