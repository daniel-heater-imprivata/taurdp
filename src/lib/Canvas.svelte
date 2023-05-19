<script>
	import { appWindow } from '@tauri-apps/api/window';
	import { emit, listen } from '@tauri-apps/api/event'
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	onMount(() => {
		console.log('on mount event');
		invoke('handle_bitmaps', { window: appWindow });
	})
	
	const result = invoke('handle_bitmaps', {    
    window: appWindow
	});

	const onUpdateCanvas= appWindow.listen(
   	 'UPDATECANVAS',
   ({event, payload}) => console.log(payload));

	function onCanvasLoad(event) {
		console.log('Loading canvas');
		invoke('handle_bitmaps', { window: appWindow });
	}
		
	function mouseDown(event) {
		appWindow.emit('event', { x: event.clientX, y: event.clientY });
		event.stopPropagation();

		// const webview = new WebviewWindow('window');
		// webview.emit('event');
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
