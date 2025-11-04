import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

export const useAwsStore = defineStore('aws', {
	state: () => ({
		profile: null,
		region: 'eu-north-1',
		profiles: [],
		clusters: [],
		services: [],
		tasks: [],
		currentCluster: null,
		currentService: null,
		currentTask: null,
		taskDetails: null,
		loading: false,
		error: null
	}),

	actions: {
		async loadProfiles() {
			try {
				this.error = null
				const profiles = await invoke('list_aws_profiles')
				this.profiles = profiles || []
				console.log('[AWS Store] Loaded profiles:', this.profiles.length)
				return this.profiles
			} catch (error) {
				console.error('[AWS Store] Failed to load profiles:', error)
				const errorMsg = error?.toString() || 'Failed to load AWS profiles'
				this.error = this.formatErrorMessage(errorMsg)
				this.profiles = []
				throw error
			}
		},

		formatErrorMessage(errorMsg) {
			// Convert backend error messages to user-friendly format
			if (errorMsg.includes('not installed') || errorMsg.includes('not found in PATH')) {
				return 'AWS CLI is not installed. Please install AWS CLI v2 from https://aws.amazon.com/cli/'
			}
			if (errorMsg.includes('configuration file not found') || errorMsg.includes('.aws/config')) {
				return 'AWS configuration file not found. Please create ~/.aws/config with your AWS SSO profiles.'
			}
			if (errorMsg.includes('profile not found')) {
				return 'AWS profile not found. Please check your ~/.aws/config file.'
			}
			if (errorMsg.includes('credentials not found') || errorMsg.includes('InvalidClientTokenId')) {
				return 'AWS credentials not found or invalid. Please sign in with SSO first.'
			}
			if (errorMsg.includes('Access denied') || errorMsg.includes('UnauthorizedOperation')) {
				return 'Access denied. Your credentials may not have permission for this operation.'
			}
			if (errorMsg.includes('timeout') || errorMsg.includes('connection')) {
				return 'Network timeout or connection error. Please check your internet connection.'
			}
			return errorMsg
		},

		async ssoLogin(profile) {
			try {
				this.loading = true
				this.error = null
				const result = await invoke('sso_login', { profile })
				this.profile = profile
				this.loading = false
				return true
			} catch (error) {
				this.loading = false
				const errorMsg = error?.toString() || 'SSO login failed. Please try again.'
				this.error = this.formatErrorMessage(errorMsg)
				throw error
			}
		},

	async loadClusters() {
		if (!this.profile || !this.region) {
			this.error = 'Profile or region not set'
			return
		}

		try {
			this.loading = true
			this.error = null
			this.clusters = await invoke('ecs_list_clusters', {
				profile: this.profile,
				region: this.region
			})
		} catch (error) {
			console.error('[AWS Store] Failed to load clusters:', error)
			const errorMsg = error?.toString() || 'Failed to load clusters'
			this.error = this.formatErrorMessage(errorMsg)
			this.clusters = []
		} finally {
			this.loading = false
		}
	},

	async loadServices(cluster) {
		if (!this.profile || !this.region || !cluster) {
			this.error = 'Profile, region, or cluster not set'
			return
		}

		try {
			this.loading = true
			this.error = null
			this.currentCluster = cluster
			this.services = await invoke('ecs_list_services', {
				profile: this.profile,
				region: this.region,
				cluster
			})
		} catch (error) {
			console.error('[AWS Store] Failed to load services:', error)
			const errorMsg = error?.toString() || 'Failed to load services'
			this.error = this.formatErrorMessage(errorMsg)
			this.services = []
		} finally {
			this.loading = false
		}
	},

	async loadTasks(cluster, service = null) {
		if (!this.profile || !this.region || !cluster) {
			this.error = 'Profile, region, or cluster not set'
			return
		}

		try {
			this.loading = true
			this.error = null
			this.currentCluster = cluster
			this.currentService = service
			this.tasks = await invoke('ecs_list_tasks', {
				profile: this.profile,
				region: this.region,
				cluster,
				service: service || null
			})
		} catch (error) {
			console.error('[AWS Store] Failed to load tasks:', error)
			const errorMsg = error?.toString() || 'Failed to load tasks'
			this.error = this.formatErrorMessage(errorMsg)
			this.tasks = []
		} finally {
			this.loading = false
		}
	},

	async loadTaskDetails(cluster, taskArn) {
		if (!this.profile || !this.region || !cluster || !taskArn) {
			this.error = 'Profile, region, cluster, or task ARN not set'
			return
		}

		try {
			this.loading = true
			this.error = null
			this.currentTask = taskArn
			this.taskDetails = await invoke('ecs_describe_tasks', {
				profile: this.profile,
				region: this.region,
				cluster,
				taskArn
			})
		} catch (error) {
			console.error('[AWS Store] Failed to load task details:', error)
			const errorMsg = error?.toString() || 'Failed to load task details'
			this.error = this.formatErrorMessage(errorMsg)
			this.taskDetails = null
		} finally {
			this.loading = false
		}
	},

		setProfile(profile) {
			this.profile = profile
		},

		setRegion(region) {
			this.region = region
		}
	}
})

