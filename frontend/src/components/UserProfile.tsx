import { useParams } from "react-router-dom";
import { createFollow, getFollowees, getFollowers } from "../api";
import { useUser } from "../hooks";
import useSWR from "swr";

const UserProfile = () => {
    const { name } = useParams();
    const user = useUser(name ?? "");
    let userName = "";
    if (user?.status === 200) {
        userName = user.data.name;
    }
    const { data: followees, mutate: mutateFollowees } = useSWR(
        ["/api/followees/{name}", userName],
        async ([_, name]) => await getFollowees({ name })
    );
    const { data: followers, mutate: mutateFollowers } = useSWR(
        ["/api/followers/{name}", userName],
        async ([_, name]) => await getFollowers({ name })
    );

    const follow = async () => {
        if (user?.status !== 200) return;
        await createFollow({ follow_to_name: userName });
        await mutateFollowees();
        await mutateFollowers();
    };

    return (
        <div>
            {user?.status === 200 ? (
                <>
                    <p className="name">{user.data.name}</p>
                    <p className="followees">
                        {followees?.status === 200 ? followees?.data.length : 0}{" "}
                        follows
                    </p>
                    <p className="followers">
                        {followers?.status === 200 ? followers?.data.length : 0}{" "}
                        followers
                    </p>
                    <button
                        onClick={() => {
                            follow().catch(console.error);
                        }}
                    >
                        Follow
                    </button>
                </>
            ) : (
                <p>Not found</p>
            )}
        </div>
    );
};

export default UserProfile;
