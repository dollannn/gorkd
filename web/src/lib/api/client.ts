import { ApiError, type ApiErrorResponse } from './errors'
import type {
	HealthResponse,
	JobResponse,
	ResearchRequest,
	ResearchResponse,
	SourcesResponse,
} from './types'

export interface ApiClientConfig {
	baseUrl: string
	timeout?: number
}

export interface ApiClient {
	startResearch(request: ResearchRequest): Promise<ResearchResponse>
	getJob(jobId: string): Promise<JobResponse>
	getSources(jobId: string): Promise<SourcesResponse>
	health(): Promise<HealthResponse>
}

const DEFAULT_TIMEOUT = 30_000

export function createApiClient(config: ApiClientConfig): ApiClient {
	const { baseUrl, timeout = DEFAULT_TIMEOUT } = config

	async function request<T>(
		path: string,
		options: RequestInit = {}
	): Promise<T> {
		const controller = new AbortController()
		const timeoutId = setTimeout(() => controller.abort(), timeout)

		const url = `${baseUrl}${path}`

		if (import.meta.env.DEV) {
			console.log(`[API] ${options.method ?? 'GET'} ${url}`)
		}

		try {
			const response = await fetch(url, {
				...options,
				signal: controller.signal,
				headers: {
					'Content-Type': 'application/json',
					...options.headers,
				},
			})

			const data = await response.json()

			if (import.meta.env.DEV) {
				console.log(`[API] ${response.status}`, data)
			}

			if (!response.ok) {
				throw ApiError.fromResponse(data as ApiErrorResponse, response.status)
			}

			return data as T
		} catch (error) {
			if (error instanceof ApiError) {
				throw error
			}
			if (error instanceof DOMException && error.name === 'AbortError') {
				throw ApiError.timeout()
			}
			throw ApiError.networkError(
				error instanceof Error ? error.message : 'Network error'
			)
		} finally {
			clearTimeout(timeoutId)
		}
	}

	return {
		async startResearch(
			researchRequest: ResearchRequest
		): Promise<ResearchResponse> {
			return request<ResearchResponse>('/v1/research', {
				method: 'POST',
				body: JSON.stringify(researchRequest),
			})
		},

		async getJob(jobId: string): Promise<JobResponse> {
			return request<JobResponse>(`/v1/jobs/${jobId}`)
		},

		async getSources(jobId: string): Promise<SourcesResponse> {
			return request<SourcesResponse>(`/v1/jobs/${jobId}/sources`)
		},

		async health(): Promise<HealthResponse> {
			return request<HealthResponse>('/v1/health')
		},
	}
}
