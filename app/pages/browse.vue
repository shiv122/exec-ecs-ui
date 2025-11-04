<template>
	<div class="min-h-screen bg-white dark:bg-gray-950">
		<!-- Common Header -->
		<CommonHeader>
			<template #left>
				<UButton
					variant="ghost"
					color="gray"
					size="sm"
					@click="goBackToProfile"
					icon="lucide:arrow-left"
				/>
					
				
				<div>
					<h1 class="text-lg font-semibold text-gray-900 dark:text-white">ECS Resources</h1>
				</div>
			</template>
			<template #actions>
				<UButton
					variant="ghost"
					color="gray"
					size="sm"
					:disabled="store.loading"
					@click="handleRefresh"
				>
					<Icon 
						:name="store.loading ? 'lucide:loader-2' : 'lucide:refresh-cw'" 
						:class="['w-4 h-4', store.loading ? 'animate-spin' : '']" 
					/>
				</UButton>
			</template>
		</CommonHeader>

		<!-- Main Content -->
		<UContainer class="py-6">
			<!-- Stepper -->
			<div class="mb-8">
				<div class="flex items-center gap-3">
					<UButton
						v-if="activeStep > 0"
						variant="ghost"
						color="gray"
						size="sm"
						:disabled="store.loading || isLoadingStep"
						class="flex-shrink-0 w-8 h-8 p-0 rounded-full hover:bg-accent-500/10 hover:text-accent-500 hover:ring-1 hover:ring-accent-500/20 transition-all"
						@click="goToPreviousStep"
					>
						<Icon name="lucide:chevron-left" class="w-4 h-4" />
					</UButton>
					<UStepper
						:items="stepperItems"
						:model-value="activeStep"
						color="accent"
						size="sm"
						@update:model-value="navigateToStep"
						class="flex-1"
					/>
				</div>
			</div>

			<!-- Content Sections -->
			<div class="space-y-6">
				<!-- Error Alert -->
				<UAlert
					v-if="store.error"
					color="red"
					variant="soft"
					icon="lucide:alert-circle"
					@close="store.error = null"
				>
					<template #title>
						<div class="font-semibold">Error</div>
					</template>
					<template #description>
						<div class="mt-1 text-sm whitespace-pre-line">{{ store.error }}</div>
					</template>
				</UAlert>

				<!-- Step 0: Clusters -->
				<div v-if="activeStep === 0">
					<div class="flex items-center justify-between mb-4">
						<h2 class="text-sm font-semibold text-gray-700 dark:text-gray-300">Select Cluster</h2>
						<UInput
							v-model="clusterSearch"
							placeholder="Search clusters..."
							size="sm"
							color="gray"
							icon="lucide:search"
							class="w-48"
						/>
					</div>
					<div v-if="isLoadingStep || (store.loading && store.clusters.length === 0)" class="flex items-center justify-center py-12">
						<div class="text-center">
							<Icon name="lucide:loader-2" class="w-8 h-8 mx-auto animate-spin text-gray-400 mb-3" />
							<p class="text-sm text-gray-500 dark:text-gray-400">Loading clusters...</p>
						</div>
					</div>
					<div v-else-if="filteredClusters.length === 0" class="text-center py-12 text-gray-500 dark:text-gray-400 text-sm">
						{{ clusterSearch ? `No clusters match "${clusterSearch}"` : 'No clusters found' }}
					</div>
					<div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
						<UCard
							v-for="cluster in filteredClusters"
							:key="cluster"
							class="cursor-pointer transition-all hover:shadow-lg hover:ring-2 hover:ring-accent-500"
							:class="selectedCluster === cluster ? 'ring-2 ring-gray-900 dark:ring-gray-100' : ''"
							@click="selectCluster(cluster)"
						>
							<div class="flex items-center gap-4">
								<div class="w-12 h-12 rounded-lg bg-gray-100 dark:bg-gray-800 flex items-center justify-center flex-shrink-0">
									<Icon name="lucide:server" class="w-6 h-6 text-gray-600 dark:text-gray-300" />
								</div>
								<div class="flex-1 min-w-0">
									<div class="font-semibold text-gray-900 dark:text-white truncate">{{ getClusterName(cluster) }}</div>
									<div class="text-xs text-gray-500 dark:text-gray-400 font-mono truncate mt-1">{{ cluster }}</div>
								</div>
							</div>
						</UCard>
					</div>
				</div>

				<!-- Step 1: Services -->
				<div v-else-if="activeStep === 1">
					<div class="flex items-center justify-between mb-4">
						<h2 class="text-sm font-semibold text-gray-700 dark:text-gray-300">Select Service</h2>
						<UInput
							v-model="serviceSearch"
							placeholder="Search services..."
							size="sm"
							color="gray"
							icon="lucide:search"
							class="w-48"
						/>
					</div>
					<div v-if="isLoadingStep || (store.loading && store.services.length === 0)" class="flex items-center justify-center py-12">
						<div class="text-center">
							<Icon name="lucide:loader-2" class="w-8 h-8 mx-auto animate-spin text-gray-400 mb-3" />
							<p class="text-sm text-gray-500 dark:text-gray-400">Loading services...</p>
						</div>
					</div>
					<div v-else-if="filteredServices.length === 0" class="text-center py-12 text-gray-500 dark:text-gray-400 text-sm">
						{{ serviceSearch ? `No services match "${serviceSearch}"` : 'No services found' }}
					</div>
					<div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
						<UCard
							v-for="service in filteredServices"
							:key="service"
							class="cursor-pointer transition-all hover:shadow-lg hover:ring-2 hover:ring-accent-500"
							:class="selectedService === service ? 'ring-2 ring-gray-900 dark:ring-gray-100' : ''"
							@click="selectService(service)"
						>
							<div class="flex items-center gap-4">
								<div class="w-12 h-12 rounded-lg bg-gray-100 dark:bg-gray-800 flex items-center justify-center flex-shrink-0">
									<Icon name="lucide:layers" class="w-6 h-6 text-gray-600 dark:text-gray-300" />
								</div>
								<div class="flex-1 min-w-0">
									<div class="font-semibold text-gray-900 dark:text-white truncate">{{ getServiceName(service) }}</div>
									<div class="text-xs text-gray-500 dark:text-gray-400 font-mono truncate mt-1">{{ service }}</div>
								</div>
							</div>
						</UCard>
					</div>
				</div>

				<!-- Step 2: Tasks -->
				<div v-else-if="activeStep === 2">
					<div class="flex items-center justify-between mb-4">
						<h2 class="text-sm font-semibold text-gray-700 dark:text-gray-300">Select Task</h2>
						<UInput
							v-model="taskSearch"
							placeholder="Search tasks..."
							size="sm"
							color="gray"
							icon="lucide:search"
							class="w-48"
						/>
					</div>
					<div v-if="isLoadingStep || (store.loading && store.tasks.length === 0)" class="flex items-center justify-center py-12">
						<div class="text-center">
							<Icon name="lucide:loader-2" class="w-8 h-8 mx-auto animate-spin text-gray-400 mb-3" />
							<p class="text-sm text-gray-500 dark:text-gray-400">Loading tasks...</p>
						</div>
					</div>
					<div v-else-if="filteredTasks.length === 0" class="text-center py-12 text-gray-500 dark:text-gray-400 text-sm">
						{{ taskSearch ? `No tasks match "${taskSearch}"` : 'No tasks found' }}
					</div>
					<div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
						<UCard
							v-for="task in filteredTasks"
							:key="task"
							class="cursor-pointer transition-all hover:shadow-lg hover:ring-2 hover:ring-accent-500"
							:class="selectedTask === task ? 'ring-2 ring-gray-900 dark:ring-gray-100' : ''"
							@click="selectTask(task)"
						>
							<div class="flex items-center gap-4">
								<div class="w-12 h-12 rounded-lg bg-gray-100 dark:bg-gray-800 flex items-center justify-center flex-shrink-0">
									<Icon name="lucide:box" class="w-6 h-6 text-gray-600 dark:text-gray-300" />
								</div>
								<div class="flex-1 min-w-0">
									<div class="font-semibold text-gray-900 dark:text-white truncate">{{ getTaskName(task) }}</div>
									<div class="text-xs text-gray-500 dark:text-gray-400 font-mono truncate mt-1">{{ task }}</div>
								</div>
							</div>
						</UCard>
					</div>
				</div>

				<!-- Step 3: Containers -->
				<div v-else-if="activeStep === 3">
					<div class="flex items-center justify-between mb-4">
						<h2 class="text-sm font-semibold text-gray-700 dark:text-gray-300">Select Container</h2>
						<UInput
							v-model="containerSearch"
							placeholder="Search containers..."
							size="sm"
							color="gray"
							icon="lucide:search"
							class="w-48"
						/>
					</div>
					<div v-if="isLoadingStep || (store.loading && !store.taskDetails)" class="flex items-center justify-center py-12">
						<div class="text-center">
							<Icon name="lucide:loader-2" class="w-8 h-8 mx-auto animate-spin text-gray-400 mb-3" />
							<p class="text-sm text-gray-500 dark:text-gray-400">Loading containers...</p>
						</div>
					</div>
					<div v-else-if="filteredContainers.length === 0" class="text-center py-12 text-gray-500 dark:text-gray-400 text-sm">
						{{ containerSearch ? `No containers match "${containerSearch}"` : 'No containers found' }}
					</div>
					<div v-else class="space-y-2">
						<div
							v-for="container in filteredContainers"
							:key="container.name"
							class="p-3 border border-gray-200 dark:border-gray-800 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-900 hover:ring-2 hover:ring-accent-500 transition-all cursor-pointer"
							:class="container.running ? '' : 'opacity-50'"
							@click="container.running && startSession(container.name)"
						>
							<div class="flex items-center justify-between gap-4">
								<div class="flex items-center gap-3 flex-1 min-w-0">
									<div class="w-2 h-2 rounded-full flex-shrink-0" :class="container.running ? 'bg-accent-500' : 'bg-gray-400'" />
									<div class="flex-1 min-w-0">
										<div class="font-semibold text-sm text-gray-900 dark:text-white truncate">{{ container.name }}</div>
										<div class="text-xs text-gray-500 dark:text-gray-400 font-mono truncate">{{ container.image }}</div>
									</div>
								</div>
								<UButton
									:disabled="!container.running"
									size="sm"
									color="gray"
									class="flex-shrink-0"
									@click.stop="startSession(container.name)"
								>
									<Icon name="lucide:terminal" class="w-4 h-4 mr-2" />
									Exec
								</UButton>
							</div>
						</div>
					</div>
				</div>
			</div>
		</UContainer>
	</div>
</template>

<script setup>
import { useAwsStore } from '~/stores/aws'

const route = useRoute()
const router = useRouter()
const store = useAwsStore()

// Local state for selected items and active step
const activeStep = ref(0)
const selectedCluster = ref(null)
const selectedService = ref(null)
const selectedTask = ref(null)
const isLoadingStep = ref(false)

// Search states
const clusterSearch = ref('')
const serviceSearch = ref('')
const taskSearch = ref('')
const containerSearch = ref('')

// Computed step based on selections
const stepperItems = computed(() => [
	{
		title: 'Cluster',
		description: 'Select a cluster',
		icon: 'lucide:server',
		disabled: false
	},
	{
		title: 'Service',
		description: 'Select a service',
		icon: 'lucide:layers',
		disabled: !selectedCluster.value
	},
	{
		title: 'Task',
		description: 'Select a task',
		icon: 'lucide:box',
		disabled: !selectedService.value
	},
	{
		title: 'Container',
		description: 'Select a container',
		icon: 'lucide:package',
		disabled: !selectedTask.value
	}
])

onMounted(async () => {
	const profile = route.query.profile
	const region = route.query.region

	if (profile) {
		store.setProfile(profile)
	}
	if (region) {
		store.setRegion(region)
	}

	await loadClusters()
})

// Navigation functions
const navigateToStep = async (step) => {
	if (store.loading || isLoadingStep.value) return
	if (stepperItems.value[step].disabled) return

	await goToStep(step)
}

const goToPreviousStep = async () => {
	if (store.loading || isLoadingStep.value) return
	
	if (activeStep.value === 3) {
		await goToStep(2)
	} else if (activeStep.value === 2) {
		await goToStep(1)
	} else if (activeStep.value === 1) {
		await goToStep(0)
	}
}

const goToStep = async (step) => {
	isLoadingStep.value = true
	
	try {
		switch (step) {
			case 0:
				// Reset everything and load clusters
				selectedCluster.value = null
				selectedService.value = null
				selectedTask.value = null
				store.currentCluster = null
				store.currentService = null
				store.currentTask = null
				store.taskDetails = null
				store.services = []
				store.tasks = []
				clusterSearch.value = ''
				serviceSearch.value = ''
				taskSearch.value = ''
				containerSearch.value = ''
				activeStep.value = 0
				await loadClusters()
				break
				
			case 1:
				// Reset to services, load services for selected cluster
				selectedService.value = null
				selectedTask.value = null
				store.currentService = null
				store.currentTask = null
				store.taskDetails = null
				store.tasks = []
				serviceSearch.value = ''
				taskSearch.value = ''
				containerSearch.value = ''
				activeStep.value = 1
				if (selectedCluster.value) {
					await loadServices(selectedCluster.value)
				}
				break
				
			case 2:
				// Reset to tasks, load tasks for selected service
				selectedTask.value = null
				store.currentTask = null
				store.taskDetails = null
				taskSearch.value = ''
				containerSearch.value = ''
				activeStep.value = 2
				if (selectedService.value && selectedCluster.value) {
					await loadTasks(selectedCluster.value, selectedService.value)
				}
				break
				
			case 3:
				// Load task details for selected task
				containerSearch.value = ''
				activeStep.value = 3
				if (selectedTask.value && selectedCluster.value) {
					await loadTaskDetails(selectedCluster.value, selectedTask.value)
				}
				break
		}
	} finally {
		isLoadingStep.value = false
	}
}

// Selection handlers
const selectCluster = async (cluster) => {
	if (store.loading || isLoadingStep.value) return
	
	// Prevent double-clicks
	if (selectedCluster.value === cluster) return
	
	selectedCluster.value = cluster
	store.currentCluster = cluster
	selectedService.value = null
	selectedTask.value = null
	store.currentService = null
	store.currentTask = null
	store.taskDetails = null
	store.services = []
	store.tasks = []
	
	await goToStep(1)
}

const selectService = async (service) => {
	if (store.loading || isLoadingStep.value) return
	
	// Prevent double-clicks
	if (selectedService.value === service) return
	
	selectedService.value = service
	store.currentService = service
	selectedTask.value = null
	store.currentTask = null
	store.taskDetails = null
	store.tasks = []
	
	await goToStep(2)
}

const selectTask = async (task) => {
	if (store.loading || isLoadingStep.value) return
	
	// Prevent double-clicks
	if (selectedTask.value === task) return
	
	selectedTask.value = task
	store.currentTask = task
	
	await goToStep(3)
}

// Data loading functions
const loadClusters = async () => {
	if (store.loading) return
	try {
		await store.loadClusters()
	} catch (error) {
		console.error('Failed to load clusters:', error)
		store.error = error?.toString() || 'Failed to load clusters'
		isLoadingStep.value = false
	}
}

const loadServices = async (cluster) => {
	if (store.loading) return
	try {
		await store.loadServices(cluster)
	} catch (error) {
		console.error('Failed to load services:', error)
		store.error = error?.toString() || 'Failed to load services'
		isLoadingStep.value = false
	}
}

const loadTasks = async (cluster, service) => {
	if (store.loading) return
	try {
		await store.loadTasks(cluster, service)
	} catch (error) {
		console.error('Failed to load tasks:', error)
		store.error = error?.toString() || 'Failed to load tasks'
		isLoadingStep.value = false
	}
}

const loadTaskDetails = async (cluster, taskArn) => {
	if (store.loading) return
	try {
		await store.loadTaskDetails(cluster, taskArn)
	} catch (error) {
		console.error('Failed to load task details:', error)
		store.error = error?.toString() || 'Failed to load task details'
		isLoadingStep.value = false
	}
}

const handleRefresh = async () => {
	if (store.loading || isLoadingStep.value) return
	
	switch (activeStep.value) {
		case 0:
			await loadClusters()
			break
		case 1:
			if (selectedCluster.value) {
				await loadServices(selectedCluster.value)
			}
			break
		case 2:
			if (selectedService.value && selectedCluster.value) {
				await loadTasks(selectedCluster.value, selectedService.value)
			}
			break
		case 3:
			if (selectedTask.value && selectedCluster.value) {
				await loadTaskDetails(selectedCluster.value, selectedTask.value)
			}
			break
	}
}

// Utility functions
const getClusterName = (arn) => {
	return arn.split('/').pop() || arn
}

const getServiceName = (arn) => {
	return arn.split('/').pop() || arn
}

const getTaskName = (arn) => {
	return arn.split('/').pop() || arn
}

const containers = computed(() => {
	if (!store.taskDetails || !store.taskDetails.tasks || store.taskDetails.tasks.length === 0) {
		return []
	}

	const task = store.taskDetails.tasks[0]
	return (task.containers || []).map(container => ({
		name: container.name,
		image: container.image,
		running: container.lastStatus === 'RUNNING'
	}))
})

// Filtered lists
const filteredClusters = computed(() => {
	if (!clusterSearch.value) return store.clusters
	const search = clusterSearch.value.toLowerCase()
	return store.clusters.filter(cluster => {
		const name = getClusterName(cluster).toLowerCase()
		const arn = cluster.toLowerCase()
		return name.includes(search) || arn.includes(search)
	})
})

const filteredServices = computed(() => {
	if (!serviceSearch.value) return store.services
	const search = serviceSearch.value.toLowerCase()
	return store.services.filter(service => {
		const name = getServiceName(service).toLowerCase()
		const arn = service.toLowerCase()
		return name.includes(search) || arn.includes(search)
	})
})

const filteredTasks = computed(() => {
	if (!taskSearch.value) return store.tasks
	const search = taskSearch.value.toLowerCase()
	return store.tasks.filter(task => {
		const name = getTaskName(task).toLowerCase()
		const arn = task.toLowerCase()
		return name.includes(search) || arn.includes(search)
	})
})

const filteredContainers = computed(() => {
	if (!containerSearch.value) return containers.value
	const search = containerSearch.value.toLowerCase()
	return containers.value.filter(container => {
		const name = container.name.toLowerCase()
		const image = container.image.toLowerCase()
		return name.includes(search) || image.includes(search)
	})
})

const goBackToProfile = () => {
	router.push('/')
}

const startSession = (containerName) => {
	router.push({
		path: '/console',
		query: {
			profile: store.profile,
			region: store.region,
			cluster: selectedCluster.value,
			task: selectedTask.value,
			container: containerName,
			shell: '/bin/bash'
		}
	})
}
</script>
