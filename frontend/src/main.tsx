import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import AuthProvider from "./components/AuthProvider";
import Home from "./components/Home";
import Login from "./components/Login";
import RequireAuth from "./components/RequireAuth";
import Signup from "./components/Signup";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <AuthProvider>
            <BrowserRouter>
                <Routes>
                    <Route
                        path="/"
                        element={
                            <RequireAuth>
                                <Home />
                            </RequireAuth>
                        }
                    />
                    <Route path="/login" element={<Login />} />
                    <Route path="/signup" element={<Signup />} />
                </Routes>
            </BrowserRouter>
        </AuthProvider>
    </React.StrictMode>
);
