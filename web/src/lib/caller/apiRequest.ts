import { API_URL } from '$lib/utils/constants';
import type { ApiError, ApiResult } from '$lib/utils/types';
import type { HttpMethod } from '@sveltejs/kit';

export async function callApi<T>(
	route: string,
	method: HttpMethod,
	body?: object
): Promise<ApiResult<T> | undefined> {
	const stringifiedBody = JSON.stringify(body);

	const headers: HeadersInit = {
		'Content-Type': 'application/json'
	};

	// if auth token, add it in
	const token = localStorage.getItem('token');
	if (token != null) {
		headers['Authorization'] = `Bearer ${token}`;
	}

	const request = await fetch(`${API_URL}${route}`, {
		method,
		headers,
		body: stringifiedBody
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
			return;
		} else {
			return {
				error: true,
				message: 'Internal error'
			} as ApiError;
		}
	}
}
