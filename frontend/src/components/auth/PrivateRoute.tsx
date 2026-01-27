import React from 'react';
import { Navigate } from 'react-router-dom';

interface PrivateRouteProps {
    children: React.ReactNode;
}

export const PrivateRoute: React.FC<PrivateRouteProps> = ({children}) =>{
    const token = localStorage.getItem("token");

    if(!token) {
        return <Navigate to="/signin" replace />
    }

    return <>{children}</>
};