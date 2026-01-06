import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { createApiClient, type ApiClient } from './client'
import { ApiError } from './errors'

describe('ApiClient', () => {
	let client: ApiClient
	let fetchSpy: ReturnType<typeof vi.fn>

	beforeEach(() => {
		client = createApiClient({ baseUrl: 'http://localhost:4000', timeout: 1000 })
		fetchSpy = vi.fn()
		vi.stubGlobal('fetch', fetchSpy)
	})

	afterEach(() => {
		vi.unstubAllGlobals()
	})

	describe('error handling', () => {
		it('throws ApiError on 400 response', async () => {
			fetchSpy.mockResolvedValue({
				ok: false,
				status: 400,
				json: () =>
					Promise.resolve({
						error: { code: 'INVALID_QUERY', message: 'Query cannot be empty' }
					})
			})

			await expect(client.startResearch({ query: '' })).rejects.toThrow(ApiError)
			await expect(client.startResearch({ query: '' })).rejects.toMatchObject({
				code: 'INVALID_QUERY',
				message: 'Query cannot be empty',
				statusCode: 400
			})
		})

		it('throws ApiError on 404 response', async () => {
			fetchSpy.mockResolvedValue({
				ok: false,
				status: 404,
				json: () =>
					Promise.resolve({
						error: { code: 'JOB_NOT_FOUND', message: 'Job not found' }
					})
			})

			await expect(client.getJob('job_invalid')).rejects.toMatchObject({
				code: 'JOB_NOT_FOUND',
				statusCode: 404
			})
		})

		it('throws ApiError on 429 response', async () => {
			fetchSpy.mockResolvedValue({
				ok: false,
				status: 429,
				json: () =>
					Promise.resolve({
						error: { code: 'RATE_LIMITED', message: 'Too many requests' }
					})
			})

			await expect(client.startResearch({ query: 'test' })).rejects.toMatchObject({
				code: 'RATE_LIMITED',
				statusCode: 429
			})
		})

		it('throws network error on fetch failure', async () => {
			fetchSpy.mockRejectedValue(new Error('Network failure'))

			await expect(client.health()).rejects.toMatchObject({
				code: 'NETWORK_ERROR',
				message: 'Network failure'
			})
		})

		it('throws timeout error on abort', async () => {
			fetchSpy.mockImplementation(
				() =>
					new Promise((_, reject) => {
						setTimeout(() => {
							const error = new DOMException('Aborted', 'AbortError')
							reject(error)
						}, 50)
					})
			)

			await expect(client.health()).rejects.toMatchObject({
				code: 'TIMEOUT'
			})
		})
	})

	describe('successful requests', () => {
		it('starts research job', async () => {
			const response = {
				job_id: 'job_abc123',
				status: 'pending',
				stream_url: '/v1/jobs/job_abc123/stream'
			}
			fetchSpy.mockResolvedValue({
				ok: true,
				status: 202,
				json: () => Promise.resolve(response)
			})

			const result = await client.startResearch({ query: 'What is X?' })
			expect(result).toEqual(response)
			expect(fetchSpy).toHaveBeenCalledWith(
				'http://localhost:4000/v1/research',
				expect.objectContaining({
					method: 'POST',
					body: JSON.stringify({ query: 'What is X?' })
				})
			)
		})

		it('gets job status', async () => {
			const response = {
				job_id: 'job_abc123',
				status: 'completed',
				query: 'What is X?',
				answer: { summary: 'X is...', detail: 'Details...', confidence: 'high' }
			}
			fetchSpy.mockResolvedValue({
				ok: true,
				status: 200,
				json: () => Promise.resolve(response)
			})

			const result = await client.getJob('job_abc123')
			expect(result).toEqual(response)
		})

		it('gets sources', async () => {
			const response = {
				sources: [{ id: 'src_001', url: 'https://example.com', title: 'Example', domain: 'example.com' }]
			}
			fetchSpy.mockResolvedValue({
				ok: true,
				status: 200,
				json: () => Promise.resolve(response)
			})

			const result = await client.getSources('job_abc123')
			expect(result).toEqual(response)
		})

		it('checks health', async () => {
			const response = { status: 'healthy', version: '0.1.0', uptime_seconds: 3600 }
			fetchSpy.mockResolvedValue({
				ok: true,
				status: 200,
				json: () => Promise.resolve(response)
			})

			const result = await client.health()
			expect(result).toEqual(response)
		})
	})
})
