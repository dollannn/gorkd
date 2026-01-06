import { browser } from '$app/environment'
import { goto } from '$app/navigation'
import { resolveRoute } from '$app/paths'
import { createApiClient, ApiError, type JobResponse } from '$lib/api'

export type ResearchState =
	| 'idle'
	| 'submitting'
	| 'streaming'
	| 'completed'
	| 'error'

const MAX_QUERY_LENGTH = 2000

const api = createApiClient({
	baseUrl: browser ? '' : 'http://localhost:4000',
})

export interface ResearchStore {
	readonly state: ResearchState
	readonly job: JobResponse | null
	readonly error: ApiError | null
	readonly query: string
	submit(query: string): Promise<void>
	retry(): Promise<void>
	reset(): void
	loadJob(jobId: string): Promise<void>
	setQuery(query: string): void
}

function createResearchStore(): ResearchStore {
	let state = $state<ResearchState>('idle')
	let job = $state<JobResponse | null>(null)
	let error = $state<ApiError | null>(null)
	let query = $state<string>('')

	async function submit(queryText: string): Promise<void> {
		if (!queryText.trim()) return
		if (queryText.length > MAX_QUERY_LENGTH) return

		state = 'submitting'
		error = null
		query = queryText

		try {
			const response = await api.startResearch({ query: queryText })
			const jobData = await api.getJob(response.job_id)
			job = jobData
			state = jobData.status === 'completed' ? 'completed' : 'streaming'
			await goto(resolveRoute('/research/[id]', { id: response.job_id }))
		} catch (err) {
			error =
				err instanceof ApiError ? err : ApiError.networkError('Unknown error')
			state = 'error'
		}
	}

	async function retry(): Promise<void> {
		if (state !== 'error' || !query) return
		await submit(query)
	}

	function reset(): void {
		state = 'idle'
		job = null
		error = null
		query = ''
	}

	async function loadJob(jobId: string): Promise<void> {
		state = 'submitting'
		error = null

		try {
			const jobData = await api.getJob(jobId)
			job = jobData
			query = jobData.query

			if (jobData.status === 'completed') {
				state = 'completed'
			} else if (jobData.status === 'failed') {
				state = 'error'
				error = new ApiError('INTERNAL_ERROR', 'Research job failed')
			} else {
				state = 'streaming'
			}
		} catch (err) {
			if (err instanceof ApiError && err.code === 'JOB_NOT_FOUND') {
				error = err
			} else {
				error =
					err instanceof ApiError ? err : ApiError.networkError('Unknown error')
			}
			state = 'error'
		}
	}

	function setQuery(newQuery: string): void {
		query = newQuery
	}

	return {
		get state() {
			return state
		},
		get job() {
			return job
		},
		get error() {
			return error
		},
		get query() {
			return query
		},
		submit,
		retry,
		reset,
		loadJob,
		setQuery,
	}
}

export const researchStore = createResearchStore()
