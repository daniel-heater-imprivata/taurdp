<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { appWindow, WebviewWindow } from '@tauri-apps/api/window';

	function mouseDown(event) {
		appWindow.emit('event', { x: event.clientX, y: event.clientY });
		event.stopPropogation();

		const webview = new WebviewWindow('window');
		webview.emit('event');
	}

	function keyDown(event) {
		invoke('keyboard_press', { s: event.key });
	}
</script>

<main>
	<div>
		<canvas
			width={800}
			height={600}
			on:mousedown={mouseDown}
			on:keydown={keyDown}
			id={'sessionCanvas'}
			tabIndex={0}
		/>
	</div>
</main>

<style>
	canvas {
		background-color: blue;
		margin-right: auto;
		margin-left: auto;
		display: block;
	}
</style>
