import { Terminal } from 'xterm'
import { FitAddon } from '@xterm/addon-fit'
import { WebLinksAddon } from '@xterm/addon-web-links'
import 'xterm/css/xterm.css'

export default defineNuxtPlugin(() => {
	return {
		provide: {
			createTerminal: (element) => {
			const terminal = new Terminal({
				convertEol: true,
				fontFamily: 'ui-monospace, SFMono-Regular, "SF Mono", Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace',
				fontSize: 14,
				padding: 8,
				theme: {
					background: '#0f172a',
					foreground: '#e2e8f0',
					cursor: '#60a5fa',
					selection: '#334155',
					black: '#1e293b',
					red: '#ef4444',
					green: '#22c55e',
					yellow: '#eab308',
					blue: '#3b82f6',
					magenta: '#a855f7',
					cyan: '#06b6d4',
					white: '#e2e8f0',
					brightBlack: '#475569',
					brightRed: '#f87171',
					brightGreen: '#4ade80',
					brightYellow: '#fbbf24',
					brightBlue: '#60a5fa',
					brightMagenta: '#c084fc',
					brightCyan: '#22d3ee',
					brightWhite: '#f1f5f9'
				},
				cursorBlink: true,
				cursorStyle: 'block',
				rows: 30,
				cols: 120
			})

				const fitAddon = new FitAddon()
				terminal.loadAddon(fitAddon)
				terminal.loadAddon(new WebLinksAddon())

				terminal.open(element)
				fitAddon.fit()

				return { terminal, fitAddon }
			}
		}
	}
})

