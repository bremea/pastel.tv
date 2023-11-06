import { accessToken } from '$lib/context/context';
import { API_URL } from '$lib/utils/constants';
import type { ApiError, ApiResult } from '$lib/utils/types';
import type { HttpMethod } from '@sveltejs/kit';
import { get } from 'svelte/store';

export async function callApi<T>(
	route: string,
	method: HttpMethod,
	body?: object,
	options?: RequestInit
): Promise<ApiResult<T>> {
	const stringifiedBody = JSON.stringify(body);

	const headers: HeadersInit = {
		'Content-Type': 'application/json'
	};

	// if auth token, add it in
	const token = get(accessToken);
	if (token) {
		headers['Authorization'] = `Bearer ${token}`;
	}

	const request = await fetch(`${API_URL}${route}`, {
		method,
		headers,
		body: stringifiedBody,
		...options
	});

	try {
		const response = (await request.json()) as ApiResult<T>;
		if (response.error) {
			return response as ApiError;
		} else {
			response.error = false;
			return response;
		}
	} catch (e) {
		if (request.status === 200) {
			return {
				error: false
			} as ApiResult<T>;
		} else {
			return {
				error: true,
				message: 'Internal error'
			} as ApiError;
		}
	}
}
