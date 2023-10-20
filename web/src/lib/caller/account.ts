import type { EmailInfo } from '$lib/utils/types';
import { callApi } from './apiRequest';

export const checkEmail = async (email: string) =>
	await callApi<EmailInfo>('/account/exists', 'POST', { email });

export const sendEmailVerify = async (email: string, name: string) =>
	await callApi<EmailInfo>('/account/verify', 'POST', { email, name });
