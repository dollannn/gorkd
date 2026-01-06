export { createApiClient, type ApiClient, type ApiClientConfig } from './client'
export {
	ApiError,
	isApiError,
	type ApiErrorCode,
	type ApiErrorResponse,
} from './errors'
export {
	createResearchStream,
	type ConnectionState,
	type ResearchStream,
	type StreamCallbacks,
	type StreamConfig,
} from './stream'
export type {
	Answer,
	AnswerEvent,
	Citation,
	CompleteEvent,
	Confidence,
	HealthResponse,
	JobMetadata,
	JobResponse,
	JobStatus,
	Progress,
	ResearchRequest,
	ResearchResponse,
	Source,
	SourceEvent,
	SourcesResponse,
	StatusEvent,
	StreamEvent,
} from './types'
