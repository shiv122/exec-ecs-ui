export default defineNuxtPlugin(async () => {
	const { useSettingsStore } = await import('~/stores/settings')
	
	// Initialize settings store on app startup
	const settingsStore = useSettingsStore()
	
	// Initialize asynchronously without blocking
	settingsStore.initialize().catch(error => {
		console.error('[Settings Plugin] Failed to initialize settings store:', error)
	})
	
	console.log('[Settings Plugin] Settings store initialization started')
})

