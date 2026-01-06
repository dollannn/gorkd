/// <reference types="@testing-library/jest-dom" />
import { render, screen, cleanup } from '@testing-library/svelte'
import { afterEach, describe, it, expect, vi } from 'vitest'
import userEvent from '@testing-library/user-event'
import ErrorState from './ErrorState.svelte'

afterEach(() => {
	cleanup()
})

describe('ErrorState', () => {
	it('renders with default generic error', () => {
		render(ErrorState)
		expect(screen.getByRole('alert')).toBeInTheDocument()
		expect(screen.getByText('Something went wrong')).toBeInTheDocument()
		expect(screen.getByText('An unexpected error occurred. Please try again.')).toBeInTheDocument()
	})

	it('renders network error type', () => {
		render(ErrorState, {
			props: {
				type: 'network',
			},
		})
		expect(screen.getByText("Couldn't connect to server")).toBeInTheDocument()
		expect(screen.getByText('Please check your internet connection and try again.')).toBeInTheDocument()
	})

	it('renders not_found error type', () => {
		render(ErrorState, {
			props: {
				type: 'not_found',
			},
		})
		expect(screen.getByText('Research not found')).toBeInTheDocument()
		expect(screen.getByText("This research job doesn't exist or may have expired.")).toBeInTheDocument()
	})

	it('renders rate_limited error type', () => {
		render(ErrorState, {
			props: {
				type: 'rate_limited',
			},
		})
		expect(screen.getByText('Too many requests')).toBeInTheDocument()
		expect(screen.getByText('Please wait a moment before trying again.')).toBeInTheDocument()
	})

	it('renders search_failed error type', () => {
		render(ErrorState, {
			props: {
				type: 'search_failed',
			},
		})
		expect(screen.getByText('Search providers unavailable')).toBeInTheDocument()
	})

	it('renders llm_failed error type', () => {
		render(ErrorState, {
			props: {
				type: 'llm_failed',
			},
		})
		expect(screen.getByText("Couldn't generate answer")).toBeInTheDocument()
		expect(screen.getByText('The AI service is temporarily unavailable.')).toBeInTheDocument()
	})

	it('uses custom title and message when provided', () => {
		render(ErrorState, {
			props: {
				title: 'Custom Title',
				message: 'Custom message here',
			},
		})
		expect(screen.getByText('Custom Title')).toBeInTheDocument()
		expect(screen.getByText('Custom message here')).toBeInTheDocument()
	})

	it('shows retry button when onRetry is provided and type is not not_found', () => {
		const handleRetry = vi.fn()
		render(ErrorState, {
			props: {
				type: 'generic',
				onRetry: handleRetry,
			},
		})
		expect(screen.getByRole('button', { name: /try again/i })).toBeInTheDocument()
	})

	it('does not show retry button for not_found type', () => {
		const handleRetry = vi.fn()
		render(ErrorState, {
			props: {
				type: 'not_found',
				onRetry: handleRetry,
			},
		})
		expect(screen.queryByRole('button', { name: /try again/i })).not.toBeInTheDocument()
	})

	it('shows new research button when onNewQuery is provided', () => {
		const handleNewQuery = vi.fn()
		render(ErrorState, {
			props: {
				onNewQuery: handleNewQuery,
			},
		})
		expect(screen.getByRole('button', { name: /new research/i })).toBeInTheDocument()
	})

	it('calls onRetry when retry button is clicked', async () => {
		const user = userEvent.setup()
		const handleRetry = vi.fn()

		render(ErrorState, {
			props: {
				onRetry: handleRetry,
			},
		})

		await user.click(screen.getByRole('button', { name: /try again/i }))
		expect(handleRetry).toHaveBeenCalledTimes(1)
	})

	it('calls onNewQuery when new research button is clicked', async () => {
		const user = userEvent.setup()
		const handleNewQuery = vi.fn()

		render(ErrorState, {
			props: {
				onNewQuery: handleNewQuery,
			},
		})

		await user.click(screen.getByRole('button', { name: /new research/i }))
		expect(handleNewQuery).toHaveBeenCalledTimes(1)
	})

	it('uses custom retry label when provided', () => {
		render(ErrorState, {
			props: {
				onRetry: vi.fn(),
				retryLabel: 'Retry Connection',
			},
		})
		expect(screen.getByRole('button', { name: /retry connection/i })).toBeInTheDocument()
	})

	it('has proper accessibility attributes', () => {
		render(ErrorState)
		const alert = screen.getByRole('alert')
		expect(alert).toHaveAttribute('aria-live', 'polite')
	})
})
