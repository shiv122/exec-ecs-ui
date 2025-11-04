export default defineAppConfig({
	app: {
		name: "ECS Exec UI",
		author: "Shivesh Tripathi",
		repo: "https://github.com/shiv122/exec-ecs-ui",
		tauriSite: "https://tauri.app",
		nuxtSite: "https://nuxt.com",
		nuxtUiSite: "https://ui4.nuxt.dev"
	},
	ui: {
		primary: 'gray',
		gray: 'slate',
		colors: {
			accent: 'accent'
		},
		stepper: {
			default: {
				color: 'accent'
			},
			color: {
				accent: {
					base: 'text-gray-400 dark:text-gray-600',
					active: 'text-accent-500',
					completed: 'text-accent-500'
				}
			}
		},
		card: {
			base: 'bg-gray-900 dark:bg-gray-950 border border-gray-800 dark:border-gray-900',
			header: {
				base: 'bg-gray-800/50 dark:bg-gray-900/50 border-b border-gray-800 dark:border-gray-900'
			},
			body: {
				base: 'bg-gray-900 dark:bg-gray-950'
			}
		},
		button: {
			default: {
				color: 'gray',
				variant: 'solid'
			},
			color: {
				gray: {
					solid: 'bg-gray-800 dark:bg-gray-800 text-white dark:text-gray-100 hover:bg-gray-700 dark:hover:bg-gray-700 shadow-sm',
					outline: 'border-2 border-gray-700 dark:border-gray-700 text-gray-300 dark:text-gray-300 hover:bg-gray-800 dark:hover:bg-gray-800',
					ghost: 'text-gray-300 dark:text-gray-300 hover:bg-gray-800 dark:hover:bg-gray-800'
				}
			}
		},
		input: {
			default: {
				color: 'gray'
			},
			color: {
				gray: {
					outline: 'bg-gray-900 dark:bg-gray-950 border-gray-700 dark:border-gray-800 text-white dark:text-white placeholder:text-gray-500 dark:placeholder:text-gray-500 focus:border-gray-600 dark:focus:border-gray-700 focus:ring-gray-600 dark:focus:ring-gray-700'
				}
			}
		},
		alert: {
			default: {
				color: 'gray'
			},
			color: {
				gray: {
					soft: 'bg-gray-800/50 dark:bg-gray-900/50 border-gray-700 dark:border-gray-800 text-gray-300 dark:text-gray-300'
				},
				red: {
					soft: 'bg-gray-900 dark:bg-gray-950 border-gray-800 dark:border-gray-900 text-gray-300 dark:text-gray-300'
				}
			}
		},
		badge: {
			default: {
				color: 'gray'
			},
			color: {
				gray: {
					solid: 'bg-gray-800 dark:bg-gray-800 text-white dark:text-gray-100',
					soft: 'bg-gray-800/50 dark:bg-gray-900/50 text-gray-300 dark:text-gray-300 border border-gray-700 dark:border-gray-800'
				}
			}
		},
		selectMenu: {
			default: {
				color: 'gray'
			}
		}
	}
});
