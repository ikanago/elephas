import { Navigate } from "react-router-dom";
import { useAuth } from "../context";

const RequireAuth = ({ children }: { children: React.ReactNode }) => {
    const { isAuthenticated } = useAuth();

    if (!isAuthenticated) {
        return <Navigate to="/signup" />
    }

    return children;
};

export default RequireAuth;
