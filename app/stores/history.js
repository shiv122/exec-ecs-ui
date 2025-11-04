import { defineStore } from 'pinia'
import { Store } from '@tauri-apps/plugin-store'

// Single store instance for persistence
let historyStoreInstance = null

const getHistoryStore = async () => {
	if (!historyStoreInstance) {
		historyStoreInstance = await Store.load('.history.dat')
	}
	return historyStoreInstance
}

export const useHistoryStore = defineStore('history', {
	state: () => ({
		commands: [],
		initialized: false,
		loading: false
	}),

	getters: {
		isEmpty: (state) => state.commands.length === 0,
		commandCount: (state) => state.commands.length
	},

	actions: {
		async initialize() {
			if (this.initialized) {
				return
			}
			
			this.loading = true
			try {
				const tauriStore = await getHistoryStore()
				const saved = await tauriStore.get('commands')
				
				if (saved && Array.isArray(saved)) {
					// Validate and filter out invalid entries
					this.commands = saved.filter(cmd => 
						cmd && 
						cmd.profile && 
						cmd.region && 
						cmd.cluster && 
						cmd.task && 
						cmd.container &&
						cmd.id &&
						cmd.timestamp
					)
					console.log('[History] Loaded', this.commands.length, 'commands')
				} else {
					this.commands = []
					console.log('[History] No saved history found')
				}
				
				this.initialized = true
			} catch (error) {
				console.error('[History] Failed to initialize:', error)
				this.commands = []
				this.initialized = true // Mark as initialized even on error to prevent retry loops
			} finally {
				this.loading = false
			}
		},

		async ensureInitialized() {
			if (!this.initialized) {
				await this.initialize()
			}
		},

		async loadHistory() {
			await this.ensureInitialized()
		},

		async saveToStorage() {
			try {
				const tauriStore = await getHistoryStore()
				await tauriStore.set('commands', this.commands)
				await tauriStore.save()
				console.log('[History] Saved', this.commands.length, 'commands to storage')
			} catch (error) {
				console.error('[History] Failed to save to storage:', error)
				throw error
			}
		},

		async addCommand(command) {
			try {
				// Ensure store is initialized
				await this.ensureInitialized()

				// Validate required fields
				if (!command.profile || !command.region || !command.cluster || !command.task || !command.container) {
					console.error('[History] Invalid command data:', command)
					return false
				}

				// Create a unique identifier for the command
				const commandId = `${command.profile}-${command.region}-${command.cluster}-${command.task}-${command.container}-${command.shell || '/bin/bash'}`
				
				// Remove duplicate if exists (move to top if already exists)
				const existingIndex = this.commands.findIndex(cmd => cmd.id === commandId)
				if (existingIndex !== -1) {
					this.commands.splice(existingIndex, 1)
				}

				// Create new command object
				const newCommand = {
					...command,
					id: commandId,
					timestamp: Date.now()
				}

				// Add to beginning
				this.commands.unshift(newCommand)

				// Get max history items from settings (try to get, but default to 10 if fails)
				let maxItems = 10
				try {
					const { useSettingsStore } = await import('./settings')
					const settingsStore = useSettingsStore()
					await settingsStore.ensureInitialized()
					maxItems = settingsStore.maxHistoryItems || 10
				} catch (error) {
					console.warn('[History] Failed to get max history items from settings, using default:', error)
				}
				
				// Keep only last N items
				if (this.commands.length > maxItems) {
					this.commands = this.commands.slice(0, maxItems)
				}

				console.log('[History] Added command:', newCommand.id, 'Total:', this.commands.length)

				// Save to persistent storage
				await this.saveToStorage()
				
				return true
			} catch (error) {
				console.error('[History] Failed to add command:', error)
				return false
			}
		},

		async deleteCommand(commandId) {
			try {
				await this.ensureInitialized()
				
				const beforeCount = this.commands.length
				this.commands = this.commands.filter(cmd => cmd.id !== commandId)
				
				if (this.commands.length < beforeCount) {
					console.log('[History] Deleted command:', commandId)
					await this.saveToStorage()
				}
			} catch (error) {
				console.error('[History] Failed to delete command:', error)
				throw error
			}
		},

		async clearHistory() {
			try {
				await this.ensureInitialized()
				
				this.commands = []
				await this.saveToStorage()
				console.log('[History] Cleared all history')
			} catch (error) {
				console.error('[History] Failed to clear history:', error)
				throw error
			}
		}
	}
})
