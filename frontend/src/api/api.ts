import axios from 'axios';
import type {
    AuthResponse, Website, ReliabilityData
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

export const createWebsite = async(payload: {name: string, url: string, check_interval: number}) => {
    const response = await api.post('/api/createwebsite', payload);
    return response.data;
};

export const fetchWebsite = async (): Promise<Website[]> => {
    const response = await api.get<Website[]>('/api/getwebsites');
    return response.data;
};

export const fetchReliability = async (websiteId: string): Promise<ReliabilityData[]> => {
    const response = await api.get<ReliabilityData[]>(`/api/websites/${websiteId}/reliability`);
        return response.data;
}