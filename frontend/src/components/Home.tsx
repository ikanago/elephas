import { home } from "../api";
import { useAuth } from "../context";
import useSWR from "swr";

const Home = () => {
    const { isAuthenticated } = useAuth();
    const { data } = useSWR("home", home);

    return <>{isAuthenticated ? <h1>{data?.name}</h1> : <h1>Not logged in</h1>}</>;
};

export default Home;
