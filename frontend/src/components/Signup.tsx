import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { signup } from "../api";

const Signup = () => {
  const [name, setName] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const navigate = useNavigate();

  const submit = async () => {
    const res = await signup({ name, password });
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
          Name:
          <input
            type="text"
            name="username"
            value={name}
            onChange={(e) => {
              setName(e.target.value);
            }}
          />
          Password:
          <input
            type="password"
            name="password"
            value={password}
            onChange={(e) => {
              setPassword(e.target.value);
            }}
          />
        </label>
        <input
          type="submit"
          value="Sign up"
          onClick={(e) => {
            e.preventDefault();
            submit().catch(console.error);
          }}
        />
      </form>
      <p className="error">{error}</p>
    </div>
  );
};

export default Signup;
