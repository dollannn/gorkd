/// <reference types="@testing-library/jest-dom" />
import { render, screen, cleanup } from '@testing-library/svelte'
import { afterEach, describe, it, expect } from 'vitest'
import Skeleton from '$lib/components/Skeleton.svelte'
import AnswerSkeleton from '$lib/components/AnswerSkeleton.svelte'
import SourceSkeleton from '$lib/components/SourceSkeleton.svelte'

afterEach(() => {
	cleanup()
})

describe('Skeleton', () => {
	it('renders with default props', () => {
		render(Skeleton)
		const skeleton = document.querySelector('.animate-pulse')
		expect(skeleton).toBeInTheDocument()
	})

	it('renders with custom width and height', () => {
		render(Skeleton, {
			props: {
				width: '100px',
				height: '20px',
			},
		})
		const skeleton = document.querySelector('.animate-pulse')
		expect(skeleton).toHaveStyle({ width: '100px', height: '20px' })
	})

	it('renders with different rounded options', () => {
		const { rerender } = render(Skeleton, {
			props: {
				rounded: 'full',
			},
		})
		expect(document.querySelector('.rounded-full')).toBeInTheDocument()

		rerender({
			rounded: 'none',
		})
		expect(document.querySelector('.rounded-full')).not.toBeInTheDocument()
	})

	it('is hidden from screen readers', () => {
		render(Skeleton)
		const skeleton = document.querySelector('.animate-pulse')
		expect(skeleton).toHaveAttribute('aria-hidden', 'true')
	})
})

describe('AnswerSkeleton', () => {
	it('renders loading status', () => {
		render(AnswerSkeleton)
		expect(screen.getByRole('status')).toBeInTheDocument()
		expect(screen.getByRole('status')).toHaveAttribute('aria-label', 'Loading answer')
	})

	it('renders multiple skeleton elements', () => {
		render(AnswerSkeleton)
		const skeletons = document.querySelectorAll('.animate-pulse')
		expect(skeletons.length).toBeGreaterThan(3)
	})
})

describe('SourceSkeleton', () => {
	it('renders loading status', () => {
		render(SourceSkeleton)
		expect(screen.getByRole('status')).toBeInTheDocument()
		expect(screen.getByRole('status')).toHaveAttribute('aria-label', 'Loading sources')
	})

	it('renders single source skeleton by default', () => {
		render(SourceSkeleton)
		const containers = document.querySelectorAll('.rounded-lg.border.p-4')
		expect(containers.length).toBe(1)
	})

	it('renders multiple source skeletons based on count prop', () => {
		render(SourceSkeleton, {
			props: {
				count: 3,
			},
		})
		const containers = document.querySelectorAll('.rounded-lg.border.p-4')
		expect(containers.length).toBe(3)
	})
})
