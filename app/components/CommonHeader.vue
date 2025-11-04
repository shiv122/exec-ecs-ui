<template>
	<div class="border-b border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-950">
		<UContainer class="py-3">
			<div class="flex items-center justify-between gap-4">
				<div class="flex items-center gap-4 min-w-0 flex-1">
					<slot name="left">
						<div class="flex items-center gap-3 min-w-0">
							<img 
								src="/logo.png" 
								alt="ECS Exec UI" 
								class="w-8 h-8 flex-shrink-0"
							/>
							<div class="min-w-0">
								<h1 class="text-lg font-semibold text-gray-900 dark:text-white">
									<slot name="title">ECS Exec UI</slot>
								</h1>
								<div v-if="store.profile && store.region" class="text-xs text-gray-500 dark:text-gray-400 font-mono truncate">
									{{ store.profile }} / {{ store.region }}
								</div>
							</div>
						</div>
					</slot>
				</div>
				<div class="flex items-center gap-2 flex-shrink-0">
					<!-- Tool Status Dropdown -->
					<UDropdownMenu :items="toolMenuItems" :ui="{ content: 'w-64' }">
						<UButton
							variant="ghost"
							color="gray"
							size="sm"
							:class="hasMissingTools ? 'text-red-500 dark:text-red-400' : 'text-green-500 dark:text-green-400'"
						>
							<Icon name="fluent:desktop-toolbox-24-regular" class="w-4 h-4 mr-1.5" />
							<span class="text-xs font-medium">Tools</span>
							<div
								class="w-2 h-2 rounded-full ml-1.5 flex-shrink-0"
								:class="hasMissingTools ? 'bg-red-500' : 'bg-green-500'"
							/>
						</UButton>
					</UDropdownMenu>
					<slot name="actions" />
					<UButton
						variant="ghost"
						color="gray"
						size="sm"
						to="/history"
						title="Command History"
					>
						<Icon name="material-symbols:deployed-code-history-outline" class="w-4 h-4" />
					</UButton>
					<UButton
						variant="ghost"
						color="gray"
						size="sm"
						to="/settings"
						title="Settings"
					>
						<Icon name="lucide:settings" class="w-4 h-4" />
					</UButton>
					<ClientOnly>
						<UButton
							variant="ghost"
							color="gray"
							size="sm"
							:icon="isDark ? 'lucide:sun' : 'lucide:moon'"
							@click="toggleTheme"
							aria-label="Toggle theme"
						/>
						<template #fallback>
							<div class="w-9 h-9" />
						</template>
					</ClientOnly>
				</div>
			</div>
		</UContainer>
	</div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core'
import { useAwsStore } from '~/stores/aws'

const colorMode = useColorMode()
const store = useAwsStore()

const toolStatus = ref([])

const hasMissingTools = computed(() => {
	return toolStatus.value.some(tool => !tool.installed)
})

const toolMenuItems = computed(() => {
	if (toolStatus.value.length === 0) {
		return [[
			{
				label: 'Checking tools...',
				icon: 'lucide:loader-2',
				type: 'label',
				disabled: true
			}
		]]
	}

	const items = []
	
	// Add header
	items.push([
		{
			label: 'Required Tools',
			type: 'label'
		}
	])

	// Add each tool as a separate item
	const toolItems = toolStatus.value.map(tool => ({
		label: tool.name,
		description: tool.installed ? (tool.version || 'Installed') : 'Not found',
		icon: tool.installed ? 'lucide:check-circle-2' : 'lucide:x-circle',
		color: tool.installed ? 'success' : 'error',
		disabled: true
	}))

	items.push(toolItems)

	return items
})

const isDark = computed({
	get() {
		return colorMode.value === 'dark'
	},
	set() {
		colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'
	}
})

const toggleTheme = () => {
	isDark.value = !isDark.value
}

const checkTools = async () => {
	try {
		const tools = await invoke('check_required_tools')
		toolStatus.value = tools || []
	} catch (error) {
		console.error('Failed to check required tools:', error)
		toolStatus.value = []
	}
}

onMounted(() => {
	checkTools()
})
</script>

