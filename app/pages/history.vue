<template>
	<div class="min-h-screen bg-white dark:bg-gray-950">
		<!-- Common Header -->
		<CommonHeader>
			<template #left>
				<UButton
					variant="ghost"
					color="gray"
					size="sm"
					@click="goBack"
				>
					<Icon name="lucide:arrow-left" class="w-4 h-4" />
				</UButton>
				<div>
					<h1 class="text-lg font-semibold text-gray-900 dark:text-white">Command History</h1>
				</div>
			</template>
			<template #actions>
				<UButton
					v-if="commands && commands.length > 0"
					variant="ghost"
					color="gray"
					size="sm"
					@click="clearHistory"
				>
					<Icon name="lucide:trash-2" class="w-4 h-4 mr-2" />
					Clear
				</UButton>
			</template>
		</CommonHeader>

		<!-- Main Content -->
		<UContainer class="py-4">
			<div v-if="!commands || commands.length === 0" class="flex flex-col items-center justify-center py-12">
				<Icon name="lucide:history" class="w-10 h-10 text-gray-400 dark:text-gray-600 mb-3" />
				<p class="text-sm text-gray-500 dark:text-gray-400">No command history yet</p>
				<p class="text-xs text-gray-400 dark:text-gray-500 mt-1">Start an exec session to see it here</p>
			</div>

			<div v-else class="space-y-1">
				<div
					v-for="command in commands"
					:key="command.id"
					class="p-2.5 border border-gray-200 dark:border-gray-800 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-900 transition-colors"
				>
					<div class="flex items-center justify-between gap-3">
						<div class="flex-1 min-w-0">
							<div class="flex items-center gap-2 mb-1">
								<Icon name="lucide:terminal" class="w-3.5 h-3.5 text-accent-500 flex-shrink-0" />
								<div class="font-semibold text-xs text-gray-900 dark:text-white truncate">
									{{ getTaskName(command.task) }} / {{ command.container }}
								</div>
							</div>
							<div class="flex items-center gap-3 text-xs text-gray-500 dark:text-gray-400 font-mono">
								<span class="truncate">{{ command.profile }}</span>
								<span class="text-gray-400 dark:text-gray-600">•</span>
								<span class="truncate">{{ command.region }}</span>
								<span class="text-gray-400 dark:text-gray-600">•</span>
								<span class="truncate">{{ getClusterName(command.cluster) }}</span>
								<span class="text-gray-400 dark:text-gray-600">•</span>
								<span class="truncate text-xs text-gray-400 dark:text-gray-500">
									{{ formatTimestamp(command.timestamp) }}
								</span>
							</div>
						</div>
						<div class="flex items-center gap-1 flex-shrink-0">
							<UButton
								variant="ghost"
								color="gray"
								size="sm"
								class="p-1.5"
								@click="runCommand(command)"
							>
								<Icon name="lucide:play" class="w-3.5 h-3.5" />
							</UButton>
							<UButton
								variant="ghost"
								color="gray"
								size="sm"
								class="p-1.5"
								@click="deleteCommand(command.id)"
							>
								<Icon name="lucide:trash-2" class="w-3.5 h-3.5" />
							</UButton>
						</div>
					</div>
				</div>
			</div>
		</UContainer>
	</div>
</template>

<script setup>
import { useHistoryStore } from '~/stores/history'
import { useRouter } from 'vue-router'

const router = useRouter()
const historyStore = useHistoryStore()

// Computed property for reactivity
const commands = computed(() => historyStore.commands)

onMounted(async () => {
	console.log('[History Page] Mounted, ensuring history is loaded...')
	await historyStore.ensureInitialized()
	console.log('[History Page] Commands loaded:', commands.value.length)
})

// Reload history when page becomes visible
onActivated(async () => {
	console.log('[History Page] Activated, refreshing history...')
	await historyStore.ensureInitialized()
})

// Watch for changes in the store
watch(() => historyStore.commands, (newCommands) => {
	console.log('[History Page] Commands updated:', newCommands.length, 'commands')
}, { deep: true })

const getTaskName = (arn) => {
	if (!arn) return ''
	return arn.split('/').pop() || arn
}

const getClusterName = (arn) => {
	if (!arn) return ''
	return arn.split('/').pop() || arn
}

const formatTimestamp = (timestamp) => {
	if (!timestamp) return 'Unknown'
	
	try {
		const date = new Date(timestamp)
		if (isNaN(date.getTime())) return 'Invalid date'
		
		const now = Date.now()
		const diff = now - timestamp
		
		const seconds = Math.floor(diff / 1000)
		const minutes = Math.floor(diff / 60000)
		const hours = Math.floor(diff / 3600000)
		const days = Math.floor(diff / 86400000)
		
		if (seconds < 60) return 'Just now'
		if (minutes < 60) return `${minutes}m ago`
		if (hours < 24) return `${hours}h ago`
		if (days < 7) return `${days}d ago`
		
		return date.toLocaleDateString()
	} catch (error) {
		console.error('[History] Error formatting timestamp:', error)
		return 'Invalid date'
	}
}

const runCommand = (command) => {
	router.push({
		path: '/console',
		query: {
			profile: command.profile,
			region: command.region,
			cluster: command.cluster,
			task: command.task,
			container: command.container,
			shell: command.shell || '/bin/bash'
		}
	})
}

const deleteCommand = async (commandId) => {
	await historyStore.deleteCommand(commandId)
}

const clearHistory = async () => {
	if (confirm('Are you sure you want to clear all history?')) {
		await historyStore.clearHistory()
	}
}

const goBack = () => {
	router.back()
}
</script>

