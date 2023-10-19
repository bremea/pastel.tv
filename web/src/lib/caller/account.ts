import type { EmailInfo } from '$lib/utils/types';
import { callApi } from './apiRequest';

export const checkEmail = async (email: string) =>
	await callApi<EmailInfo>('/account/exists', 'POST', { email: email });
