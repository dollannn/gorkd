import { browser } from '$app/environment'
import { createApiClient, type HealthResponse } from '$lib/api'

export type HealthStatus = 'unknown' | 'healthy' | 'unhealthy' | 'checking'

const POLL_INTERVAL_MS = 30_000
const HEALTH_TIMEOUT_MS = 5_000

const api = createApiClient({
	baseUrl: browser ? '' : 'http://localhost:4000',
	timeout: HEALTH_TIMEOUT_MS,
})

export interface HealthStore {
	readonly status: HealthStatus
	readonly isAvailable: boolean
	readonly lastChecked: Date | null
	readonly version: string | null
	readonly error: string | null
	check(): Promise<void>
	startPolling(): void
	stopPolling(): void
}

function createHealthStore(): HealthStore {
	let status = $state<HealthStatus>('unknown')
	let lastChecked = $state<Date | null>(null)
	let version = $state<string | null>(null)
	let error = $state<string | null>(null)

	let pollInterval: ReturnType<typeof setInterval> | null = null

	const isAvailable = $derived(status === 'healthy')

	async function check(): Promise<void> {
		if (!browser) return

		status = 'checking'
		error = null

		try {
			const response: HealthResponse = await api.health()
			status = response.status === 'healthy' ? 'healthy' : 'unhealthy'
			version = response.version
			lastChecked = new Date()
		} catch (err) {
			status = 'unhealthy'
			error = err instanceof Error ? err.message : 'Failed to connect to API'
			lastChecked = new Date()
		}
	}

	function startPolling(): void {
		if (!browser) return
		if (pollInterval) return

		check()
		pollInterval = setInterval(check, POLL_INTERVAL_MS)
	}

	function stopPolling(): void {
		if (pollInterval) {
			clearInterval(pollInterval)
			pollInterval = null
		}
	}

	return {
		get status() {
			return status
		},
		get isAvailable() {
			return isAvailable
		},
		get lastChecked() {
			return lastChecked
		},
		get version() {
			return version
		},
		get error() {
			return error
		},
		check,
		startPolling,
		stopPolling,
	}
}

export const healthStore = createHealthStore()
