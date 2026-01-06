export type ApiErrorCode =
	| 'INVALID_QUERY'
	| 'JOB_NOT_FOUND'
	| 'RATE_LIMITED'
	| 'SEARCH_FAILED'
	| 'LLM_FAILED'
	| 'INTERNAL_ERROR'
	| 'NETWORK_ERROR'
	| 'TIMEOUT'

export interface ApiErrorResponse {
	error: {
		code: string
		message: string
		details?: Record<string, unknown>
	}
}

export class ApiError extends Error {
	readonly code: ApiErrorCode
	readonly statusCode?: number
	readonly details?: Record<string, unknown>

	constructor(
		code: ApiErrorCode,
		message: string,
		statusCode?: number,
		details?: Record<string, unknown>
	) {
		super(message)
		this.name = 'ApiError'
		this.code = code
		this.statusCode = statusCode
		this.details = details
	}

	static fromResponse(response: ApiErrorResponse, statusCode: number): ApiError {
		const { code, message, details } = response.error
		return new ApiError(code as ApiErrorCode, message, statusCode, details)
	}

	static networkError(message: string): ApiError {
		return new ApiError('NETWORK_ERROR', message)
	}

	static timeout(message = 'Request timed out'): ApiError {
		return new ApiError('TIMEOUT', message)
	}
}

export function isApiError(error: unknown): error is ApiError {
	return error instanceof ApiError
}
