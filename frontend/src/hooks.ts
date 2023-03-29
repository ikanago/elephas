import useSWR from "swr";
import { getPostsOfMe, getUserProfile, me } from "./api";

export const useMe = () => {
    const { data } = useSWR("/api/me", me);
    return data;
};

export const useUser = (name: string) => {
    const { data } = useSWR(
        ["/api/users/{name}", name],
        async ([_, name]) => await getUserProfile({ name })
    );
    return data;
};

export const useMyPosts = () => {
    const { data, mutate } = useSWR("/api/posts", getPostsOfMe);
    return {
        res: data,
        mutate,
    };
};
