<template>
	<div class="min-h-screen bg-white dark:bg-gray-950 transition-colors">
		<!-- Common Header -->
		<CommonHeader>
			<template #title>ECS Exec UI</template>
		</CommonHeader>

		<!-- Main Content -->
		<div class="min-h-screen flex items-center justify-center p-6">
			<div class="w-full max-w-md">
				<!-- Header -->
				<div class="text-center mb-8">
					<div class="flex items-center justify-center gap-4 mb-4">
						<div class="w-14 h-14 rounded-xl bg-gray-900 dark:bg-gray-800 flex items-center justify-center">
							<Icon name="lucide:terminal" class="w-7 h-7 text-white" />
						</div>
						<div class="text-left">
							<h1 class="text-4xl font-black text-gray-900 dark:text-white">ECS EXEC</h1>
							<p class="text-sm text-gray-500 dark:text-gray-400 font-mono">terminal interface v2.0</p>
						</div>
					</div>
				</div>

				<!-- Card Content -->
				<UCard>
					<div class="space-y-6">
						<!-- Error Alert -->
						<UAlert
							v-if="store.error"
							color="red"
							variant="soft"
							icon="lucide:alert-circle"
						>
							<template #title>
								<div class="font-semibold">Error</div>
							</template>
							<template #description>
								<div class="mt-1 text-sm whitespace-pre-line">{{ store.error }}</div>
							</template>
						</UAlert>

						<!-- Profile Selection -->
						<div class="space-y-3">
							<div class="flex items-center justify-between">
								<label class="block text-sm font-semibold text-gray-700 dark:text-gray-300">AWS Profile</label>
								<UInput
									v-model="profileSearch"
									placeholder="Search profiles..."
									size="sm"
									color="gray"
									icon="lucide:search"
									class="w-48"
								/>
							</div>
							<div v-if="loadingProfiles" class="flex items-center justify-center py-8">
								<Icon name="lucide:loader-2" class="w-5 h-5 animate-spin text-gray-400" />
							</div>
							<div v-else-if="filteredProfiles.length > 0" class="space-y-2 max-h-64 overflow-y-auto pr-2">
								<UButton
									v-for="profile in filteredProfiles"
									:key="profile"
									:variant="selectedProfile === profile ? 'solid' : 'outline'"
									color="gray"
									block
									class="justify-start h-auto py-3 px-4 font-mono text-sm"
									@click="selectedProfile = profile"
								>
									<div class="flex items-center gap-3 w-full">
										<div class="w-2 h-2 rounded-full flex-shrink-0"
											:class="selectedProfile === profile ? 'bg-accent-500' : 'bg-gray-400 dark:bg-gray-600'">
										</div>
										<span class="font-semibold">{{ profile }}</span>
										<Icon v-if="selectedProfile === profile" name="lucide:check-circle-2" class="w-4 h-4 ml-auto flex-shrink-0 text-accent-500" />
									</div>
								</UButton>
							</div>
							<p v-else-if="profileSearch && filteredProfiles.length === 0" class="text-sm text-gray-500 dark:text-gray-400 text-center py-8">
								No profiles match "{{ profileSearch }}"
							</p>
							<p v-else class="text-sm text-gray-500 dark:text-gray-400 text-center py-8">
								No profiles found
							</p>
						</div>

						<!-- Region Input -->
						<div class="space-y-3">
							<label class="block text-sm font-semibold text-gray-700 dark:text-gray-300">Default Region</label>
							<UInput
								v-model="region"
								placeholder="eu-north-1"
								icon="lucide:globe"
								size="lg"
								color="gray"
								class="font-mono w-full"
							/>
						</div>

						<!-- Action Button -->
						<div class="space-y-3">
							<UButton
								:disabled="!selectedProfile || !region || store.loading"
								:loading="store.loading"
								block
								size="xl"
								color="gray"
								class="font-bold"
								@click="handleLogin"
							>
								<template v-if="!store.loading">
									<Icon name="lucide:log-in" class="w-5 h-5 mr-2" />
									Sign in
								</template>
							</UButton>
							<UButton
								v-if="store.loading"
								variant="ghost"
								color="gray"
								block
								size="sm"
								@click="cancelLogin"
							>
								<Icon name="lucide:x" class="w-4 h-4 mr-2" />
								Cancel
							</UButton>
						</div>
					</div>
				</UCard>
			</div>
		</div>
	</div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core'
import { Store } from '@tauri-apps/plugin-store'
import { useAwsStore } from '~/stores/aws'

const store = useAwsStore()
const router = useRouter()

const selectedProfile = ref(null)
const region = ref('eu-north-1')
const loadingProfiles = ref(false)
const profileSearch = ref('')

const filteredProfiles = computed(() => {
	if (!profileSearch.value) return store.profiles
	const search = profileSearch.value.toLowerCase()
	return store.profiles.filter(profile => profile.toLowerCase().includes(search))
})

onMounted(async () => {
	loadingProfiles.value = true
	
	// Initialize settings store
	const { useSettingsStore } = await import('~/stores/settings')
	const settingsStore = useSettingsStore()
	await settingsStore.ensureInitialized()
	
	await store.loadProfiles()
	
	// Load defaults from settings store
	if (settingsStore.defaultProfile && store.profiles.includes(settingsStore.defaultProfile)) {
		selectedProfile.value = settingsStore.defaultProfile
	}
	if (settingsStore.defaultRegion) {
		region.value = settingsStore.defaultRegion
	}
	
	// Also check legacy settings for backward compatibility
	if (!selectedProfile.value || !region.value) {
		try {
			const tauriStore = await Store.load('.settings.dat')
			const lastProfile = await tauriStore.get('lastProfile')
			const lastRegion = await tauriStore.get('lastRegion')
			
			if (!selectedProfile.value && lastProfile && store.profiles.includes(lastProfile)) {
				selectedProfile.value = lastProfile
			}
			if (!region.value && lastRegion) {
				region.value = lastRegion
			}
		} catch (error) {
			console.error('Failed to load legacy preferences:', error)
		}
	}
	
	loadingProfiles.value = false
})

const loginTimeout = ref(null)

const handleLogin = async () => {
	try {
		// Save selected profile and region
		try {
			const tauriStore = await Store.load('.settings.dat')
			await tauriStore.set('lastProfile', selectedProfile.value)
			await tauriStore.set('lastRegion', region.value)
			await tauriStore.save()
		} catch (error) {
			console.error('Failed to save preferences:', error)
		}

		// Set a timeout to prevent UI from getting stuck
		loginTimeout.value = setTimeout(() => {
			if (store.loading) {
				store.loading = false
				store.error = 'Login timeout. Please try again or complete the SSO login in your browser.'
			}
		}, 300000) // 5 minutes timeout

		await store.ssoLogin(selectedProfile.value)
		
		if (loginTimeout.value) {
			clearTimeout(loginTimeout.value)
			loginTimeout.value = null
		}

		store.setRegion(region.value)
		await router.push({ path: '/browse', query: { profile: selectedProfile.value, region: region.value } })
	} catch (error) {
		console.error('Login failed:', error)
		if (loginTimeout.value) {
			clearTimeout(loginTimeout.value)
			loginTimeout.value = null
		}
		// Error is already set in the store
	}
}

const cancelLogin = async () => {
	if (loginTimeout.value) {
		clearTimeout(loginTimeout.value)
		loginTimeout.value = null
	}
	
	try {
		if (selectedProfile.value) {
			await invoke('cancel_sso_login', { profile: selectedProfile.value })
		}
	} catch (error) {
		console.error('Failed to cancel login:', error)
	}
	
	store.loading = false
	store.error = 'Login cancelled. You can try again.'
}
</script>
