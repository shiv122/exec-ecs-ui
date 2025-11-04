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
					<h1 class="text-lg font-semibold text-gray-900 dark:text-white">Settings</h1>
				</div>
			</template>
			<template #actions>
				<UButton
					variant="ghost"
					color="gray"
					size="sm"
					@click="resetSettings"
					:disabled="settingsStore.loading"
				>
					<Icon name="lucide:rotate-ccw" class="w-4 h-4 mr-2" />
					Reset
				</UButton>
			</template>
		</CommonHeader>

		<!-- Main Content -->
		<UContainer class="py-4">
			<div v-if="settingsStore.loading" class="flex items-center justify-center py-12">
				<Icon name="lucide:loader-2" class="w-6 h-6 animate-spin text-gray-400" />
			</div>

			<div v-else class="space-y-1">
				<!-- Error Message -->
				<UAlert
					v-if="saveError"
					color="red"
					variant="soft"
					icon="lucide:alert-circle"
					@close="saveError = null"
					class="mb-4"
				>
					<template #description>
						<div class="text-sm">{{ saveError }}</div>
					</template>
				</UAlert>

				<!-- Default Profile -->
				<div class="p-3 border border-gray-200 dark:border-gray-800 rounded-lg">
					<div class="flex items-center justify-between gap-4">
						<div class="flex items-center gap-3 flex-1 min-w-0">
							<Icon name="lucide:user" class="w-4 h-4 text-gray-500 dark:text-gray-400 flex-shrink-0" />
							<div class="flex-1 min-w-0">
								<div class="text-xs font-semibold text-gray-900 dark:text-white">Default Profile</div>
								<div class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">AWS profile to use on startup</div>
							</div>
						</div>
						<div class="flex-shrink-0 w-64">
							<div v-if="loadingProfiles" class="flex items-center justify-center py-2">
								<Icon name="lucide:loader-2" class="w-4 h-4 animate-spin text-gray-400" />
							</div>
							<USelectMenu
								v-else
								v-model="localSettings.defaultProfile"
								:items="profileOptions"
								placeholder="None"
								search-input
								color="gray"
								size="sm"
								class="w-full font-mono"
								@update:model-value="debouncedSave"
							/>
						</div>
					</div>
				</div>

				<!-- Default Region -->
				<div class="p-3 border border-gray-200 dark:border-gray-800 rounded-lg">
					<div class="flex items-center justify-between gap-4">
						<div class="flex items-center gap-3 flex-1 min-w-0">
							<Icon name="lucide:globe" class="w-4 h-4 text-gray-500 dark:text-gray-400 flex-shrink-0" />
							<div class="flex-1 min-w-0">
								<div class="text-xs font-semibold text-gray-900 dark:text-white">Default Region</div>
								<div class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">AWS region for ECS operations</div>
							</div>
						</div>
						<div class="flex-shrink-0 w-64">
							<UInput
								v-model="localSettings.defaultRegion"
								placeholder="eu-north-1"
								icon="lucide:globe"
								color="gray"
								size="sm"
								class="font-mono w-full"
								@update:model-value="debouncedSave"
							/>
						</div>
					</div>
				</div>

				<!-- Default Shell -->
				<div class="p-3 border border-gray-200 dark:border-gray-800 rounded-lg">
					<div class="flex items-center justify-between gap-4">
						<div class="flex items-center gap-3 flex-1 min-w-0">
							<Icon name="lucide:terminal" class="w-4 h-4 text-gray-500 dark:text-gray-400 flex-shrink-0" />
							<div class="flex-1 min-w-0">
								<div class="text-xs font-semibold text-gray-900 dark:text-white">Default Shell</div>
								<div class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">Shell command for exec sessions</div>
							</div>
						</div>
						<div class="flex-shrink-0 w-64">
							<UInput
								v-model="localSettings.defaultShell"
								placeholder="/bin/bash"
								icon="lucide:terminal"
								color="gray"
								size="sm"
								class="font-mono w-full"
								@update:model-value="debouncedSave"
							/>
						</div>
					</div>
				</div>

				<!-- Terminal Font Size -->
				<div class="p-3 border border-gray-200 dark:border-gray-800 rounded-lg">
					<div class="flex items-center justify-between gap-4">
						<div class="flex items-center gap-3 flex-1 min-w-0">
							<Icon name="lucide:monitor" class="w-4 h-4 text-gray-500 dark:text-gray-400 flex-shrink-0" />
							<div class="flex-1 min-w-0">
								<div class="text-xs font-semibold text-gray-900 dark:text-white">Terminal Font Size</div>
								<div class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">{{ localSettings.terminalFontSize }}px</div>
							</div>
						</div>
						<div class="flex-shrink-0 w-64">
							<input
								v-model.number="localSettings.terminalFontSize"
								type="range"
								min="10"
								max="20"
								step="1"
								class="w-full h-1.5 bg-gray-200 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer accent-accent-500"
								@input="debouncedSave"
							/>
						</div>
					</div>
				</div>

				<!-- Max History Items -->
				<div class="p-3 border border-gray-200 dark:border-gray-800 rounded-lg">
					<div class="flex items-center justify-between gap-4">
						<div class="flex items-center gap-3 flex-1 min-w-0">
							<Icon name="lucide:history" class="w-4 h-4 text-gray-500 dark:text-gray-400 flex-shrink-0" />
							<div class="flex-1 min-w-0">
								<div class="text-xs font-semibold text-gray-900 dark:text-white">Max History Items</div>
								<div class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">{{ localSettings.maxHistoryItems }} items</div>
							</div>
						</div>
						<div class="flex-shrink-0 w-64">
							<input
								v-model.number="localSettings.maxHistoryItems"
								type="range"
								min="5"
								max="50"
								step="5"
								class="w-full h-1.5 bg-gray-200 dark:bg-gray-800 rounded-lg appearance-none cursor-pointer accent-accent-500"
								@input="debouncedSave"
							/>
						</div>
					</div>
				</div>

				<!-- Auto-load Last Profile -->
				<div class="p-3 border border-gray-200 dark:border-gray-800 rounded-lg">
					<div class="flex items-center justify-between gap-4">
						<div class="flex items-center gap-3 flex-1 min-w-0">
							<Icon name="lucide:refresh-cw" class="w-4 h-4 text-gray-500 dark:text-gray-400 flex-shrink-0" />
							<div class="flex-1 min-w-0">
								<div class="text-xs font-semibold text-gray-900 dark:text-white">Auto-load Last Profile</div>
								<div class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">Load last used profile on startup</div>
							</div>
						</div>
						<div class="flex-shrink-0">
							<UToggle
								v-model="localSettings.autoLoadLastProfile"
								color="accent"
								size="sm"
								@update:model-value="debouncedSave"
							/>
						</div>
					</div>
				</div>

				<!-- Auto-load Last Region -->
				<div class="p-3 border border-gray-200 dark:border-gray-800 rounded-lg">
					<div class="flex items-center justify-between gap-4">
						<div class="flex items-center gap-3 flex-1 min-w-0">
							<Icon name="lucide:refresh-cw" class="w-4 h-4 text-gray-500 dark:text-gray-400 flex-shrink-0" />
							<div class="flex-1 min-w-0">
								<div class="text-xs font-semibold text-gray-900 dark:text-white">Auto-load Last Region</div>
								<div class="text-xs text-gray-500 dark:text-gray-400 mt-0.5">Load last used region on startup</div>
							</div>
						</div>
						<div class="flex-shrink-0">
							<UToggle
								v-model="localSettings.autoLoadLastRegion"
								color="accent"
								size="sm"
								@update:model-value="debouncedSave"
							/>
						</div>
					</div>
				</div>
			</div>
		</UContainer>
	</div>
</template>

<script setup>
import { useSettingsStore } from '~/stores/settings'
import { useAwsStore } from '~/stores/aws'
import { useRouter } from 'vue-router'
import { useDebounceFn } from '@vueuse/core'
import { sendNotification } from '@tauri-apps/plugin-notification'

const router = useRouter()
const settingsStore = useSettingsStore()
const awsStore = useAwsStore()

const saveError = ref(null)
const loadingProfiles = ref(false)

// Local copy of settings for editing
const localSettings = ref({
	defaultProfile: null,
	defaultRegion: 'eu-north-1',
	defaultShell: '/bin/bash',
	terminalFontSize: 14,
	maxHistoryItems: 10,
	autoLoadLastProfile: true,
	autoLoadLastRegion: true
})

// Profile options for dropdown - convert to array of strings for USelectMenu
const profileOptions = computed(() => {
	if (!awsStore || !awsStore.profiles) {
		console.log('[Settings Page] AWS store or profiles not available')
		return []
	}
	const profiles = Array.isArray(awsStore.profiles) ? awsStore.profiles : []
	console.log('[Settings Page] Profile options computed:', profiles.length, profiles)
	// USelectMenu works with simple arrays of strings
	return profiles
})

// Watch for profile changes
watch(() => awsStore.profiles, (newProfiles) => {
	console.log('[Settings Page] Profiles updated:', newProfiles?.length || 0)
}, { deep: true })

// Load settings on mount
onMounted(async () => {
	await settingsStore.ensureInitialized()
	
	// Load profiles if not already loaded
	if (!awsStore.profiles || awsStore.profiles.length === 0) {
		loadingProfiles.value = true
		try {
			await awsStore.loadProfiles()
			console.log('[Settings Page] Profiles loaded:', awsStore.profiles?.length || 0)
		} catch (error) {
			console.error('[Settings Page] Failed to load profiles:', error)
			// Show error but don't block the page
			if (error?.toString().includes('AWS CLI') || error?.toString().includes('configuration file')) {
				saveError.value = 'Failed to load AWS profiles. ' + (awsStore.error || error?.toString())
			}
		} finally {
			loadingProfiles.value = false
		}
	}
	
	// Copy settings to local state
	localSettings.value = {
		defaultProfile: settingsStore.defaultProfile,
		defaultRegion: settingsStore.defaultRegion,
		defaultShell: settingsStore.defaultShell,
		terminalFontSize: settingsStore.terminalFontSize,
		maxHistoryItems: settingsStore.maxHistoryItems,
		autoLoadLastProfile: settingsStore.autoLoadLastProfile,
		autoLoadLastRegion: settingsStore.autoLoadLastRegion
	}
})

// Debounced save function
const debouncedSave = useDebounceFn(async () => {
	try {
		saveError.value = null
		const success = await settingsStore.updateSettings(localSettings.value)
		
		if (success) {
			// Show notification instead of alert
			try {
				await sendNotification({
					title: 'Settings Saved',
					body: 'Your settings have been saved successfully'
				})
			} catch (notifError) {
				console.error('[Settings Page] Failed to send notification:', notifError)
			}
		} else {
			saveError.value = 'Failed to save settings. Please try again.'
		}
	} catch (error) {
		console.error('[Settings Page] Failed to save:', error)
		saveError.value = error?.toString() || 'Failed to save settings.'
	}
}, 500)

const resetSettings = async () => {
	if (!confirm('Are you sure you want to reset all settings to defaults?')) {
		return
	}
	
	try {
		await settingsStore.resetToDefaults()
		
		// Update local state
		localSettings.value = {
			defaultProfile: settingsStore.defaultProfile,
			defaultRegion: settingsStore.defaultRegion,
			defaultShell: settingsStore.defaultShell,
			terminalFontSize: settingsStore.terminalFontSize,
			maxHistoryItems: settingsStore.maxHistoryItems,
			autoLoadLastProfile: settingsStore.autoLoadLastProfile,
			autoLoadLastRegion: settingsStore.autoLoadLastRegion
		}
		
		// Show notification instead of alert
		try {
			await sendNotification({
				title: 'Settings Reset',
				body: 'All settings have been reset to defaults'
			})
		} catch (notifError) {
			console.error('[Settings Page] Failed to send notification:', notifError)
		}
	} catch (error) {
		console.error('[Settings Page] Failed to reset:', error)
		saveError.value = error?.toString() || 'Failed to reset settings.'
	}
}

const goBack = () => {
	router.back()
}
</script>
