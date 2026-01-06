export type JobStatus =
	| 'pending'
	| 'planning'
	| 'searching'
	| 'fetching'
	| 'synthesizing'
	| 'completed'
	| 'failed'

export type Confidence = 'high' | 'medium' | 'low' | 'insufficient'

export interface Source {
	id: string
	url: string
	title: string
	domain: string
	published_at?: string
	content_preview?: string
	relevance_score?: number
	used_in_citations?: boolean
}

export interface Citation {
	claim: string
	source_id: string
	quote?: string
}

export interface Answer {
	summary: string
	detail: string
	confidence: Confidence
	limitations?: string[]
}

export interface Progress {
	stage: string
	message: string
	progress?: number
	sources_found?: number
}

export interface JobMetadata {
	created_at: string
	completed_at?: string
	duration_ms?: number
	sources_considered?: number
	cached?: boolean
}

export interface ResearchRequest {
	query: string
}

export interface ResearchResponse {
	job_id: string
	status: JobStatus
	stream_url: string
}

export interface JobResponse {
	job_id: string
	status: JobStatus
	query: string
	progress?: Progress
	answer?: Answer
	citations?: Citation[]
	sources?: Source[]
	metadata?: JobMetadata
}

export interface SourcesResponse {
	sources: Source[]
}

export interface HealthResponse {
	status: 'healthy' | 'degraded' | 'unhealthy'
	version: string
	uptime_seconds: number
}

export interface StatusEvent {
	stage: string
	message: string
	progress?: number
	sources_count?: number
}

export interface SourceEvent {
	id: string
	url: string
	title: string
	relevance: number
}

export interface AnswerEvent {
	summary: string
	confidence: Confidence
}

export interface CompleteEvent {
	job_id: string
	duration_ms: number
}

export type StreamEvent =
	| { type: 'status'; data: StatusEvent }
	| { type: 'source'; data: SourceEvent }
	| { type: 'answer'; data: AnswerEvent }
	| { type: 'complete'; data: CompleteEvent }
