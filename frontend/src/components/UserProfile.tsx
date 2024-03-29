import { Link, useParams } from "react-router-dom";
import { createFollow, deleteFollow } from "../api";
import {
  useFollers as useFollewers,
  useFollowees,
  useMe,
  useUser,
} from "../hooks";

const UserProfile = () => {
  const { name } = useParams();
  const name2 = name ?? "";
  const user = useUser(name2);
  let userName = "";
  if (user?.status === 200) {
    userName = user.data.name;
  }
  const { res: followees, mutate: mutateFollowees } = useFollowees(userName);
  const { res: followers, mutate: mutateFollowers } = useFollewers(userName);

  const me = useMe();
  let isFollowing = false;
  if (followers?.status === 200 && me?.status === 200) {
    isFollowing = followers.data.some(
      (follower) => follower.name === me.data.name,
    );
  }

  const follow = async () => {
    if (user?.status !== 200 || me?.status !== 200) return;
    await createFollow({
      follow_from_name: me.data.name,
      follow_to_name: name2,
    });
    await mutateFollowees();
    await mutateFollowers();
  };

  const unfollow = async () => {
    if (user?.status !== 200 || me?.status !== 200) return;
    await deleteFollow({
      follow_from_name: me.data.name,
      follow_to_name: name2,
    });
    await mutateFollowees();
    await mutateFollowers();
  };

  return (
    <div>
      {user?.status === 200 ? (
        <>
          <p className="name">{user.data.name}</p>
          <p className="displayName">{user.data.display_name}</p>
          <p className="avatarUrl">{user.data.avatar_url}</p>
          <p className="followees">
            {followees?.status === 200 ? followees?.data.length : 0} follows
          </p>
          <p className="followers">
            {followers?.status === 200 ? followers?.data.length : 0} followers
          </p>
          {isFollowing ? (
            <button
              type="submit"
              className="unfollow"
              onClick={() => {
                unfollow().catch(console.error);
              }}
            >
              Unfollow
            </button>
          ) : (
            <button
              type="submit"
              className="follow"
              onClick={() => {
                follow().catch(console.error);
              }}
            >
              Follow
            </button>
          )}
          <p className="summary">{user.data.summary}</p>
          <Link
            to="/settings/profile"
            className="edit"
            style={{
              display:
                me?.status === 200 && user.data.name !== me?.data.name
                  ? "none"
                  : undefined,
            }}
          >
            Edit Profile
          </Link>
        </>
      ) : (
        <p>Not found</p>
      )}
    </div>
  );
};

export default UserProfile;
