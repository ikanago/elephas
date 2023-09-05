import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import Home from "./components/Home";
import Login from "./components/Login";
import Settings from "./components/Settings";
import Signup from "./components/Signup";
import UserProfile from "./components/UserProfile";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/login" element={<Login />} />
        <Route path="/signup" element={<Signup />} />
        <Route path="/users/:name" element={<UserProfile />} />
        <Route path="/settings/profile" element={<Settings />} />
      </Routes>
    </BrowserRouter>
  </React.StrictMode>,
);
