import { useAuth } from "../context";

const Home = () => {
    const { isAuthenticated } = useAuth();

    return (
        <>
            {isAuthenticated ? <h1>Home</h1> : <h1>Not logged in</h1>}
        </>
    )
};

export default Home;
