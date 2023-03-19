import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { login } from "../api";
import { useAuth } from "../context";

const Login = () => {
    const [name, setName] = useState("");
    const [password, setPassword] = useState("");
    const [error, setError] = useState("");
    const { authenticate } = useAuth();
    const navigate = useNavigate();

    const submit = async () => {
        try {
            await login({ name, password });
            authenticate(() => {
                navigate("/");
            });
        } catch (err) {
            if (err instanceof Error) {
                setError(err.message);
            }
        }
    };

    return (
        <div>
            <form>
                <label>
                    Name:
                    <input
                        type="text"
                        name="username"
                        value={name}
                        onChange={e => {
                            setName(e.target.value);
                        }}
                    />
                    Password:
                    <input
                        type="password"
                        name="password"
                        value={password}
                        onChange={e => {
                            setPassword(e.target.value);
                        }}
                    />
                </label>
                <input
                    type="submit"
                    value="Log in"
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

export default Login;
