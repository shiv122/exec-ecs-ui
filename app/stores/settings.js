import { defineStore } from 'pinia'
import { Store } from '@tauri-apps/plugin-store'

// Single store instance for persistence
let settingsStoreInstance = null

const getSettingsStore = async () => {
	if (!settingsStoreInstance) {
		settingsStoreInstance = await Store.load('.settings.dat')
	}
	return settingsStoreInstance
}

export const useSettingsStore = defineStore('settings', {
	state: () => ({
		// User preferences
		defaultProfile: null,
		defaultRegion: 'eu-north-1',
		defaultShell: '/bin/bash',
		terminalFontSize: 14,
		maxHistoryItems: 10,
		autoLoadLastProfile: true,
		autoLoadLastRegion: true,
		
		// Internal state
		initialized: false,
		loading: false
	}),

	getters: {
		hasDefaults: (state) => {
			return !!(state.defaultProfile || state.defaultRegion !== 'eu-north-1')
		}
	},

	actions: {
		async initialize() {
			if (this.initialized) {
				return
			}
			
			this.loading = true
			try {
				const tauriStore = await getSettingsStore()
				
				// Load all settings
				const defaultProfile = await tauriStore.get('defaultProfile')
				const defaultRegion = await tauriStore.get('defaultRegion')
				const defaultShell = await tauriStore.get('defaultShell')
				const terminalFontSize = await tauriStore.get('terminalFontSize')
				const maxHistoryItems = await tauriStore.get('maxHistoryItems')
				const autoLoadLastProfile = await tauriStore.get('autoLoadLastProfile')
				const autoLoadLastRegion = await tauriStore.get('autoLoadLastRegion')
				
				if (defaultProfile !== null) this.defaultProfile = defaultProfile
				if (defaultRegion) this.defaultRegion = defaultRegion
				if (defaultShell) this.defaultShell = defaultShell
				if (terminalFontSize) this.terminalFontSize = terminalFontSize
				if (maxHistoryItems) this.maxHistoryItems = maxHistoryItems
				if (autoLoadLastProfile !== null) this.autoLoadLastProfile = autoLoadLastProfile
				if (autoLoadLastRegion !== null) this.autoLoadLastRegion = autoLoadLastRegion
				
				// Also load legacy settings for backward compatibility
				if (!defaultProfile) {
					const lastProfile = await tauriStore.get('lastProfile')
					if (lastProfile) {
						this.defaultProfile = lastProfile
					}
				}
				if (defaultRegion === 'eu-north-1') {
					const lastRegion = await tauriStore.get('lastRegion')
					if (lastRegion) {
						this.defaultRegion = lastRegion
					}
				}
				
				console.log('[Settings] Loaded settings:', {
					defaultProfile: this.defaultProfile,
					defaultRegion: this.defaultRegion,
					defaultShell: this.defaultShell,
					terminalFontSize: this.terminalFontSize,
					maxHistoryItems: this.maxHistoryItems
				})
				
				this.initialized = true
			} catch (error) {
				console.error('[Settings] Failed to initialize:', error)
				this.initialized = true // Mark as initialized even on error
			} finally {
				this.loading = false
			}
		},

		async ensureInitialized() {
			if (!this.initialized) {
				await this.initialize()
			}
		},

		async saveToStorage() {
			try {
				const tauriStore = await getSettingsStore()
				await tauriStore.set('defaultProfile', this.defaultProfile)
				await tauriStore.set('defaultRegion', this.defaultRegion)
				await tauriStore.set('defaultShell', this.defaultShell)
				await tauriStore.set('terminalFontSize', this.terminalFontSize)
				await tauriStore.set('maxHistoryItems', this.maxHistoryItems)
				await tauriStore.set('autoLoadLastProfile', this.autoLoadLastProfile)
				await tauriStore.set('autoLoadLastRegion', this.autoLoadLastRegion)
				await tauriStore.save()
				console.log('[Settings] Saved settings to storage')
			} catch (error) {
				console.error('[Settings] Failed to save to storage:', error)
				throw error
			}
		},

		async updateSettings(updates) {
			try {
				await this.ensureInitialized()
				
				// Apply updates
				if (updates.defaultProfile !== undefined) this.defaultProfile = updates.defaultProfile
				if (updates.defaultRegion !== undefined) this.defaultRegion = updates.defaultRegion
				if (updates.defaultShell !== undefined) this.defaultShell = updates.defaultShell
				if (updates.terminalFontSize !== undefined) this.terminalFontSize = updates.terminalFontSize
				if (updates.maxHistoryItems !== undefined) this.maxHistoryItems = updates.maxHistoryItems
				if (updates.autoLoadLastProfile !== undefined) this.autoLoadLastProfile = updates.autoLoadLastProfile
				if (updates.autoLoadLastRegion !== undefined) this.autoLoadLastRegion = updates.autoLoadLastRegion
				
				// Save to storage
				await this.saveToStorage()
				
				return true
			} catch (error) {
				console.error('[Settings] Failed to update settings:', error)
				return false
			}
		},

		async resetToDefaults() {
			try {
				this.defaultProfile = null
				this.defaultRegion = 'eu-north-1'
				this.defaultShell = '/bin/bash'
				this.terminalFontSize = 14
				this.maxHistoryItems = 10
				this.autoLoadLastProfile = true
				this.autoLoadLastRegion = true
				
				await this.saveToStorage()
				console.log('[Settings] Reset to defaults')
			} catch (error) {
				console.error('[Settings] Failed to reset settings:', error)
				throw error
			}
		}
	}
})

