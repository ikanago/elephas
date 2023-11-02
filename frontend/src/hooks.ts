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

export const useUser = (user_name: string) => {
  const { data } = useSWR(
    ["/api/users/{user_name}", user_name],
    async ([_, user_name]) => await getUserProfile({ user_name }),
  );
  return data;
};

export const useFollowees = (user_name: string) => {
  const { data, mutate } = useSWR(
    user_name !== "" ? ["/api/followees/{user_name}", user_name] : undefined,
    async ([_, user_name]) => await getFollowees({ user_name }),
  );
  return { res: data, mutate };
};

export const useFollers = (user_name: string) => {
  const { data, mutate } = useSWR(
    user_name !== "" ? ["/api/followers/{user_name}", user_name] : undefined,
    async ([_, user_name]) => await getFollowers({ user_name }),
  );
  return { res: data, mutate };
};

export const useMyPosts = () => {
  const { data, mutate } = useSWR("/api/posts", getPostsOfMe);
  return { res: data, mutate };
};
