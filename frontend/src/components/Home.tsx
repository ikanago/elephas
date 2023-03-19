import { useMe } from "../hooks";
import { useAuth } from "../context";

const Home = () => {
    const { isAuthenticated } = useAuth();
    // TODO: create API "/api/me" that returns the user's name that logged in now.
    const me = useMe();

    return (
        <>
            {isAuthenticated ? (
                <h1>{me?.unwrap().name}</h1>
            ) : (
                <h1>Not logged in</h1>
            )}
        </>
    );
};

export default Home;
