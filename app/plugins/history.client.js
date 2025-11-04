export default defineNuxtPlugin(async () => {
	const { useHistoryStore } = await import('~/stores/history')
	
	// Initialize history store on app startup
	const historyStore = useHistoryStore()
	
	// Initialize asynchronously without blocking
	historyStore.initialize().catch(error => {
		console.error('[History Plugin] Failed to initialize history store:', error)
	})
	
	console.log('[History Plugin] History store initialization started')
})

