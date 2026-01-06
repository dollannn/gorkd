import { error } from '@sveltejs/kit'
import type { PageLoad } from './$types'

export const load: PageLoad = ({ params }) => {
	const { id } = params

	if (!id || !id.startsWith('job_')) {
		error(404, { message: 'Invalid job ID' })
	}

	return {
		jobId: id,
	}
}
