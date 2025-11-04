<template>
	<div class="h-screen w-screen bg-white dark:bg-gray-950 flex flex-col overflow-hidden">
		<!-- Top Bar -->
		<div class="border-b border-gray-200 dark:border-gray-800 px-4 py-2 flex items-center justify-between gap-3 flex-shrink-0">
			<div class="flex items-center gap-3 min-w-0 flex-1">
				<UButton
					variant="ghost"
					color="gray"
					size="sm"
					class="flex-shrink-0"
					@click="goBack"
				>
					<Icon name="lucide:arrow-left" class="w-4 h-4" />
				</UButton>
				<div class="min-w-0 flex-1">
					<div class="font-semibold text-sm text-gray-900 dark:text-white truncate">
						{{ getTaskName(params.task) }} / {{ params.container }}
					</div>
					<div class="text-xs text-gray-500 dark:text-gray-400 font-mono truncate">
						{{ params.cluster }}
					</div>
				</div>
			</div>
			<div class="flex items-center gap-1 flex-shrink-0">
				<UButton
					variant="ghost"
					color="gray"
					size="sm"
					class="flex-shrink-0"
					@click="copyTaskArn"
					title="Copy Task ARN"
				>
					<Icon name="lucide:copy" class="w-4 h-4" />
				</UButton>
				<UButton
					variant="ghost"
					color="gray"
					size="sm"
					class="flex-shrink-0"
					:disabled="!sessionActive"
					@click="killSession"
					title="Kill Session"
				>
					<Icon name="lucide:x" class="w-4 h-4" />
				</UButton>
			</div>
		</div>

		<!-- Terminal Container -->
		<div class="flex-1 min-h-0 overflow-hidden">
			<div id="xterm-container" class="h-full w-full px-4 pt-4 pb-8 box-border" />
		</div>
	</div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useRoute, useRouter } from 'vue-router'
import { useClipboard } from '@vueuse/core'
import { useHistoryStore } from '~/stores/history'

const route = useRoute()
const router = useRouter()
const { $createTerminal } = useNuxtApp()
const { copy } = useClipboard()
const historyStore = useHistoryStore()

const params = computed(() => ({
	profile: route.query.profile,
	region: route.query.region,
	cluster: route.query.cluster,
	task: route.query.task,
	container: route.query.container,
	shell: route.query.shell || '/bin/bash'
}))

const sessionActive = ref(false)
const sessionId = ref(`session-${Date.now()}`)
let terminal = null
let fitAddon = null
let unlistenData = null
let unlistenError = null
let unlistenExit = null
let handleResize = null

const getTaskName = (arn) => {
	if (!arn) return ''
	return arn.split('/').pop() || arn
}

const copyTaskArn = async () => {
	if (params.value.task) {
		await copy(params.value.task)
	}
}

const goBack = () => {
	router.back()
}

const killSession = async () => {
	try {
		await invoke('close_exec_session', { sessionId: sessionId.value })
		sessionActive.value = false
		if (terminal) {
			terminal.write('\r\n\r\n[Session terminated]\r\n')
		}
	} catch (error) {
		console.error('Failed to kill session:', error)
	}
}

onMounted(async () => {
	const container = document.getElementById('xterm-container')
	if (!container) return

	const termSetup = $createTerminal(container)
	terminal = termSetup.terminal
	fitAddon = termSetup.fitAddon

	// Handle window resize
	handleResize = () => {
		if (fitAddon) {
			fitAddon.fit()
		}
	}
	window.addEventListener('resize', handleResize)

	// Handle terminal input
	terminal.onData(async (data) => {
		if (sessionActive.value) {
			try {
				await invoke('write_exec_stdin', {
					sessionId: sessionId.value,
					data
				})
			} catch (error) {
				console.error('Failed to write to terminal:', error)
			}
		}
	})

	// Listen for terminal output
	unlistenData = await listen(`term:data:${sessionId.value}`, (event) => {
		if (terminal) {
			terminal.write(event.payload || '')
		}
	})

	unlistenError = await listen(`term:error:${sessionId.value}`, (event) => {
		if (terminal) {
			terminal.write(`\r\n[ERROR] ${event.payload}\r\n`)
		}
	})

	unlistenExit = await listen(`term:exit:${sessionId.value}`, () => {
		sessionActive.value = false
		if (terminal) {
			terminal.write('\r\n\r\n[Session ended]\r\n')
		}
	})

		// Start the exec session
		try {
			await invoke('start_exec_session', {
				sessionId: sessionId.value,
				profile: params.value.profile,
				region: params.value.region,
				cluster: params.value.cluster,
				task: params.value.task,
				container: params.value.container,
				shellCmd: params.value.shell
			})
			sessionActive.value = true
			terminal.write('\r\n[Connecting to container...]\r\n')
			
			// Save to history (don't await to avoid blocking)
			historyStore.addCommand({
				profile: params.value.profile,
				region: params.value.region,
				cluster: params.value.cluster,
				task: params.value.task,
				container: params.value.container,
				shell: params.value.shell || '/bin/bash'
			}).then(success => {
				if (success) {
					console.log('[Console] Command saved to history successfully')
				} else {
					console.warn('[Console] Failed to save command to history')
				}
			}).catch(error => {
				console.error('[Console] Error saving to history:', error)
			})
		} catch (error) {
			terminal.write(`\r\n[ERROR] Failed to start session: ${error}\r\n`)
			console.error('Failed to start exec session:', error)
		}
})

onBeforeUnmount(() => {
	if (handleResize && window) {
		window.removeEventListener('resize', handleResize)
	}
	if (unlistenData) unlistenData()
	if (unlistenError) unlistenError()
	if (unlistenExit) unlistenExit()
	if (sessionActive.value) {
		invoke('close_exec_session', { sessionId: sessionId.value }).catch(console.error)
	}
	if (terminal) {
		terminal.dispose()
	}
})
</script>
