import { useParams } from "react-router-dom";
import { useUser } from "../hooks";

const UserProfile = () => {
    const { name } = useParams();
    const res = useUser(name ?? "");

    return (
        <div>
            {res?.status === 200 ? <p>{res.data.name}</p> : <p>Not found</p>}
        </div>
    );
};

export default UserProfile;
