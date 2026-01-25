import axios from 'axios';
import type {
    AuthResponse
} from '../types/types';
const API_BASE_URL = 'http://localhost:3000';

export const api = axios.create({
    baseURL: API_BASE_URL,
    headers: {
        'Content-Type': 'application/json',
    },
});

api.interceptors.request.use(
    (config) => {
        const token = localStorage.getItem('token');

        if(token) {
            config.headers.Authorization = `Bearer ${token}`;
        }

        return config;
    },
    (error) => {
        return Promise.reject(error);
    }
);

export const authRegister = async(email: string, password: string, username: string): Promise<AuthResponse> => {
    const response = await api.post<AuthResponse>('/api/createuser', {email, password, username});
    return response.data;
};

export const authLogin = async (email: string, password: string): Promise<AuthResponse> => {
    const response = await api.post<AuthResponse>('/api/signin', {email, password});
    return response.data;
};
