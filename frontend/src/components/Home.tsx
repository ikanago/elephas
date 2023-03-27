import { useMe, useMyPosts } from "../hooks";
import { useState } from "react";
import { createPost } from "../api";

const Home = () => {
    const [content, setContent] = useState("");
    const [error, setError] = useState("");
    const me = useMe();
    const { data: posts, mutate } = useMyPosts();

    const submit = async () => {
        const res = await createPost({ content });
        console.log(res);
        if (res.status === 204) {
            setContent("");
            await mutate();
        } else {
            setError(res.data.error);
        }
    };

    return (
        <>
            {me?.status === 200 ? <h1>{me.data.name}</h1> : <h1>Not logged in</h1>}
            <form>
                <label>
                    Post:
                    <input
                        type="text"
                        name="content"
                        value={content}
                        onChange={e => { setContent(e.target.value); }}
                    />
                </label>
                <input
                    type="submit"
                    value="Post"
                    onClick={e => {
                        e.preventDefault();
                        submit().catch(console.error);
                    }}
                />
            </form>
            <p className="error">{error}</p>
            <div className="timeline">
                {posts?.status === 200 ?
                    posts?.data.map(post => (
                        <div className="post" key={post.id}>
                            <p>{post.content}</p>
                        </div>
                    )) : <p>Loading...</p>}
            </div>
        </>
    );
};

export default Home;
