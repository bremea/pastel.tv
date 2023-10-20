import { API_URL } from '$lib/utils/constants';
import type { ApiError, ApiResult } from '$lib/utils/types';
import type { HttpMethod } from '@sveltejs/kit';

export async function callApi<T>(
	route: string,
	method: HttpMethod,
	body?: object
): Promise<ApiResult<T> | undefined> {
	const stringifiedBody = JSON.stringify(body);
	const request = await fetch(`${API_URL}${route}`, {
		method,
		body: stringifiedBody,
		headers: {
			'Content-Type': 'application/json'
		}
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
