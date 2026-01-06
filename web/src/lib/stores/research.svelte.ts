import { browser } from '$app/environment'
import { goto } from '$app/navigation'
import { resolveRoute } from '$app/paths'
import {
	createApiClient,
	createResearchStream,
	ApiError,
	type AnswerEvent,
	type ConnectionState,
	type JobResponse,
	type ResearchStream,
	type SourceEvent,
	type StatusEvent,
} from '$lib/api'

export type ResearchState =
	| 'idle'
	| 'submitting'
	| 'streaming'
	| 'completed'
	| 'error'

export type PipelineStage =
	| 'pending'
	| 'planning'
	| 'searching'
	| 'fetching'
	| 'synthesizing'
	| 'completed'

const MAX_QUERY_LENGTH = 2000

const api = createApiClient({
	baseUrl: browser ? '' : 'http://localhost:4000',
})

export interface StreamingSource extends SourceEvent {
	receivedAt: number
}

export interface StreamingProgress {
	stage: PipelineStage
	message: string
	progress?: number
	sourcesCount?: number
}

export interface StreamingAnswer {
	summary: string
	confidence: string
}

export interface ResearchStore {
	readonly state: ResearchState
	readonly job: JobResponse | null
	readonly error: ApiError | null
	readonly query: string
	readonly connectionState: ConnectionState
	readonly streamingSources: StreamingSource[]
	readonly streamingProgress: StreamingProgress | null
	readonly streamingAnswer: StreamingAnswer | null
	submit(query: string): Promise<void>
	retry(): Promise<void>
	reset(): void
	loadJob(jobId: string): Promise<void>
	setQuery(query: string): void
	connectStream(jobId: string): void
	disconnectStream(): void
}

function createResearchStore(): ResearchStore {
	let state = $state<ResearchState>('idle')
	let job = $state<JobResponse | null>(null)
	let error = $state<ApiError | null>(null)
	let query = $state<string>('')
	let connectionState = $state<ConnectionState>('disconnected')
	let streamingSources = $state<StreamingSource[]>([])
	let streamingProgress = $state<StreamingProgress | null>(null)
	let streamingAnswer = $state<StreamingAnswer | null>(null)

	let currentStream: ResearchStream | null = null

	function handleStatusEvent(status: StatusEvent): void {
		streamingProgress = {
			stage: status.stage as PipelineStage,
			message: status.message,
			progress: status.progress,
			sourcesCount: status.sources_count,
		}
	}

	function handleSourceEvent(source: SourceEvent): void {
		const newSource: StreamingSource = {
			...source,
			receivedAt: Date.now(),
		}
		streamingSources = [...streamingSources, newSource]
	}

	function handleAnswerEvent(answer: AnswerEvent): void {
		streamingAnswer = {
			summary: answer.summary,
			confidence: answer.confidence,
		}
	}

	async function handleCompleteEvent(): Promise<void> {
		if (!job?.job_id) return

		try {
			const finalJob = await api.getJob(job.job_id)
			job = finalJob
			state = 'completed'
			streamingProgress = {
				stage: 'completed',
				message: 'Research complete',
			}
		} catch (err) {
			error =
				err instanceof ApiError
					? err
					: ApiError.networkError('Failed to fetch final results')
			state = 'error'
		}
	}

	function handleConnectionChange(newState: ConnectionState): void {
		connectionState = newState

		if (newState === 'failed') {
			error = new ApiError(
				'NETWORK_ERROR',
				'Lost connection to research stream'
			)
			state = 'error'
		}
	}

	function handleStreamError(err: Error): void {
		error = new ApiError('NETWORK_ERROR', err.message)
	}

	function connectStream(jobId: string): void {
		disconnectStream()

		currentStream = createResearchStream(
			jobId,
			{
				onStatus: handleStatusEvent,
				onSource: handleSourceEvent,
				onAnswer: handleAnswerEvent,
				onComplete: handleCompleteEvent,
				onError: handleStreamError,
				onConnectionChange: handleConnectionChange,
			},
			{
				baseUrl: browser ? '' : 'http://localhost:4000',
			}
		)

		currentStream.connect()
	}

	function disconnectStream(): void {
		if (currentStream) {
			currentStream.disconnect()
			currentStream = null
		}
	}

	async function submit(queryText: string): Promise<void> {
		if (!queryText.trim()) return
		if (queryText.length > MAX_QUERY_LENGTH) return

		state = 'submitting'
		error = null
		query = queryText
		streamingSources = []
		streamingProgress = null
		streamingAnswer = null

		try {
			const response = await api.startResearch({ query: queryText })
			const jobData = await api.getJob(response.job_id)
			job = jobData
			state = jobData.status === 'completed' ? 'completed' : 'streaming'

			if (state === 'streaming') {
				connectStream(response.job_id)
			}

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
		disconnectStream()
		state = 'idle'
		job = null
		error = null
		query = ''
		streamingSources = []
		streamingProgress = null
		streamingAnswer = null
	}

	async function loadJob(jobId: string): Promise<void> {
		state = 'submitting'
		error = null
		streamingSources = []
		streamingProgress = null
		streamingAnswer = null

		try {
			const jobData = await api.getJob(jobId)
			job = jobData
			query = jobData.query

			if (jobData.status === 'completed') {
				state = 'completed'
				streamingProgress = {
					stage: 'completed',
					message: 'Research complete',
				}
			} else if (jobData.status === 'failed') {
				state = 'error'
				error = new ApiError('INTERNAL_ERROR', 'Research job failed')
			} else {
				state = 'streaming'
				streamingProgress = {
					stage: jobData.status as PipelineStage,
					message: jobData.progress?.message ?? 'Researching...',
					progress: jobData.progress?.progress,
					sourcesCount: jobData.progress?.sources_found,
				}
				connectStream(jobId)
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
		get connectionState() {
			return connectionState
		},
		get streamingSources() {
			return streamingSources
		},
		get streamingProgress() {
			return streamingProgress
		},
		get streamingAnswer() {
			return streamingAnswer
		},
		submit,
		retry,
		reset,
		loadJob,
		setQuery,
		connectStream,
		disconnectStream,
	}
}

export const researchStore = createResearchStore()
