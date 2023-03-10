import { useEffect } from "react";
import { config } from "../config";
import { useAuth } from "../context";

const Home = () => {
    const { isAuthenticated } = useAuth();

    useEffect(() => {
        (async () => {
            try {
                const res = await fetch(`${config.api}/`, {
                    method: "GET",
                    mode: "cors",
                    credentials: "include",
                });
                const json = await res.json();
                console.log(json);
            } catch (err) {
                console.error(err);
            }
        })();
    })

    return <>{isAuthenticated ? <h1>Home</h1> : <h1>Not logged in</h1>}</>;
};

export default Home;
