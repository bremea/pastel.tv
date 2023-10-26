import type { LoginMethod, User } from '$lib/utils/types';
import { callApi } from './apiRequest';

export const checkEmail = async (email: string) =>
	await callApi<EmailInfo>('/account/exists', 'POST', { email });

export const sendEmailVerify = async (email: string, name: string) =>
	await callApi<undefined>('/account/verify', 'POST', { email, name });

export const register = async (email: string, name: string, password: string, otp: string) =>
	await callApi<LoginResult>('/account/new', 'POST', { email, name, password, otp });

export const login = async (email: string, password: string) =>
	await callApi<LoginResult>('/account/login', 'POST', { email, password });

export const getAccessToken = async () =>
	await callApi<LoginResult>('/account/token', 'GET', undefined, { credentials: 'include' });

export const getMe = async () => await callApi<User>('/account', 'GET');

export interface EmailInfo {
	login_method: LoginMethod;
	new_account: boolean;
}

export interface LoginResult {
	user_id: string;
	access_token: string;
}
