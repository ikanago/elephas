import { useMe, useMyPosts } from "../hooks";
import { useState } from "react";
import { createPost } from "../api";

const Home = () => {
    const [content, setContent] = useState("");
    const [error, setError] = useState("");
    const me = useMe();
    const { data: posts, mutate } = useMyPosts();

    const submit = async () => {
        const r = await createPost({ content });
        if (r.ok) {
            setContent("");
            mutate();
        } else {
            setError(r.val.error);
        }
    };

    return (
        <>
            <h1>{me?.unwrap().name}</h1>
            <form>
                <label>
                    Post:
                    <input
                        type="text"
                        name="content"
                        value={content}
                        onChange={e => setContent(e.target.value)}
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
                {posts?.unwrap().map(post => (
                    <div className="post" key={post.id}>
                        <p>{post.content}</p>
                    </div>
                ))}
            </div>
        </>
    );
};

export default Home;
