import { useEffect } from "react";
import { home } from "../api";
import { useAuth } from "../context";

const Home = () => {
    const { isAuthenticated } = useAuth();

    useEffect(() => {
        (async () => {
            const res = await home();
            const json = await res.json();
            console.log(json);
        })().catch(console.error);
    });

    return <>{isAuthenticated ? <h1>Home</h1> : <h1>Not logged in</h1>}</>;
};

export default Home;
