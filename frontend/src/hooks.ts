import useSWR from "swr";
import {
    getFollowees,
    getFollowers,
    getPostsOfMe,
    getUserProfile,
    me,
} from "./api";

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

export const useFollowees = (name: string) => {
    const { data, mutate } = useSWR(
        name !== "" ? ["/api/followees/{name}", name] : undefined,
        async ([_, name]) => await getFollowees({ name })
    );
    return { res: data, mutate };
};

export const useFollers = (name: string) => {
    const { data, mutate } = useSWR(
        name !== "" ? ["/api/followers/{name}", name] : undefined,
        async ([_, name]) => await getFollowers({ name })
    );
    return { res: data, mutate };
};

export const useMyPosts = () => {
    const { data, mutate } = useSWR("/api/posts", getPostsOfMe);
    return { res: data, mutate };
};
