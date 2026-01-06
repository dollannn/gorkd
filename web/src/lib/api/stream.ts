import type {
	AnswerEvent,
	CompleteEvent,
	SourceEvent,
	StatusEvent,
} from './types'

export type ConnectionState =
	| 'disconnected'
	| 'connecting'
	| 'connected'
	| 'reconnecting'
	| 'failed'

export interface StreamCallbacks {
	onStatus?: (status: StatusEvent) => void
	onSource?: (source: SourceEvent) => void
	onAnswer?: (answer: AnswerEvent) => void
	onComplete?: (meta: CompleteEvent) => void
	onError?: (error: Error) => void
	onConnectionChange?: (state: ConnectionState) => void
}

export interface ResearchStream {
	connect(): void
	disconnect(): void
	readonly connectionState: ConnectionState
}

export interface StreamConfig {
	baseUrl: string
	maxRetries?: number
	retryDelays?: number[]
}

const DEFAULT_MAX_RETRIES = 3
const DEFAULT_RETRY_DELAYS = [1000, 2000, 5000]

export function createResearchStream(
	jobId: string,
	callbacks: StreamCallbacks,
	config: StreamConfig
): ResearchStream {
	const {
		baseUrl,
		maxRetries = DEFAULT_MAX_RETRIES,
		retryDelays = DEFAULT_RETRY_DELAYS,
	} = config

	let eventSource: EventSource | null = null
	let connectionState: ConnectionState = 'disconnected'
	let retryCount = 0
	let retryTimeoutId: ReturnType<typeof setTimeout> | null = null

	function setConnectionState(state: ConnectionState): void {
		connectionState = state
		callbacks.onConnectionChange?.(state)
	}

	function parseEventData<T>(data: string): T | null {
		try {
			return JSON.parse(data) as T
		} catch {
			console.error('[SSE] Failed to parse event data:', data)
			return null
		}
	}

	function handleOpen(): void {
		retryCount = 0
		setConnectionState('connected')

		if (import.meta.env.DEV) {
			console.log('[SSE] Connected to stream')
		}
	}

	function handleError(): void {
		if (eventSource?.readyState === EventSource.CLOSED) {
			scheduleReconnect()
		}
	}

	function scheduleReconnect(): void {
		if (retryCount >= maxRetries) {
			setConnectionState('failed')
			callbacks.onError?.(new Error('Max reconnection attempts reached'))
			return
		}

		setConnectionState('reconnecting')
		const delay = retryDelays[Math.min(retryCount, retryDelays.length - 1)]
		retryCount++

		if (import.meta.env.DEV) {
			console.log(
				`[SSE] Reconnecting in ${delay}ms (attempt ${retryCount}/${maxRetries})`
			)
		}

		retryTimeoutId = setTimeout(() => {
			retryTimeoutId = null
			connect()
		}, delay)
	}

	function setupEventListeners(): void {
		if (!eventSource) return

		eventSource.onopen = handleOpen
		eventSource.onerror = handleError

		eventSource.addEventListener('status', (event: MessageEvent) => {
			const data = parseEventData<StatusEvent>(event.data)
			if (data) {
				callbacks.onStatus?.(data)
			}
		})

		eventSource.addEventListener('source', (event: MessageEvent) => {
			const data = parseEventData<SourceEvent>(event.data)
			if (data) {
				callbacks.onSource?.(data)
			}
		})

		eventSource.addEventListener('answer', (event: MessageEvent) => {
			const data = parseEventData<AnswerEvent>(event.data)
			if (data) {
				callbacks.onAnswer?.(data)
			}
		})

		eventSource.addEventListener('complete', (event: MessageEvent) => {
			const data = parseEventData<CompleteEvent>(event.data)
			if (data) {
				callbacks.onComplete?.(data)
				disconnect()
			}
		})
	}

	function connect(): void {
		if (eventSource) {
			eventSource.close()
		}

		if (retryTimeoutId) {
			clearTimeout(retryTimeoutId)
			retryTimeoutId = null
		}

		setConnectionState('connecting')
		const url = `${baseUrl}/v1/jobs/${jobId}/stream`

		if (import.meta.env.DEV) {
			console.log('[SSE] Connecting to:', url)
		}

		eventSource = new EventSource(url)
		setupEventListeners()
	}

	function disconnect(): void {
		if (retryTimeoutId) {
			clearTimeout(retryTimeoutId)
			retryTimeoutId = null
		}

		if (eventSource) {
			eventSource.close()
			eventSource = null
		}

		retryCount = 0
		setConnectionState('disconnected')

		if (import.meta.env.DEV) {
			console.log('[SSE] Disconnected')
		}
	}

	return {
		connect,
		disconnect,
		get connectionState() {
			return connectionState
		},
	}
}
