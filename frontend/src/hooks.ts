import useSWR from "swr";
import { getPostsOfMe, me } from "./api";

export const useMe = () => {
    const { data } = useSWR("/api/me", me);
    return data;
};

export const useMyPosts = () => {
    const { data, mutate } = useSWR("/api/posts", getPostsOfMe);
    return {
        data,
        mutate
    };
}
