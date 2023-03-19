import useSWR from "swr";
import { me } from "./api";

export const useMe = () => {
    const { data } = useSWR("/api/me", me);
    return data;
};
