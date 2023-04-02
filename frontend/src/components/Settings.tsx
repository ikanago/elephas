import { useState } from "react";
import { updateMe } from "../api";
import { useNavigate } from "react-router-dom";

const Settings = () => {
    const [displayName, setDisplayName] = useState("");
    const [description, setDescription] = useState("");
    const [avatarUrl, setAvatarUrl] = useState("");
    const [error, setError] = useState("");
    const navigate = useNavigate();

    const submit = async () => {
        const res = await updateMe({
            display_name: displayName,
            description,
            avatar_url: avatarUrl,
        });
        if (res.status === 204) {
            navigate("/");
        } else {
            setError(res.data.error);
        }
    };

    return (
        <div>
            <form>
                <label>
                    Display Name:
                    <input
                        type="text"
                        name="displayName"
                        value={displayName}
                        onChange={e => {
                            setDisplayName(e.target.value);
                        }}
                    />
                    Description:
                    <input
                        type="text"
                        name="description"
                        value={description}
                        onChange={e => {
                            setDescription(e.target.value);
                        }}
                    />
                    Avatar URL:
                    <input
                        type="text"
                        name="avatarUrl"
                        value={avatarUrl}
                        onChange={e => {
                            setAvatarUrl(e.target.value);
                        }}
                    />
                </label>
                <input
                    type="submit"
                    value="Update"
                    onClick={e => {
                        e.preventDefault();
                        submit().catch(console.error);
                    }}
                />
            </form>
            <p className="error">{error}</p>
        </div>
    );
};

export default Settings;
