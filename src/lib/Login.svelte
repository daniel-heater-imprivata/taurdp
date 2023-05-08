<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let server = '';
	let port = 3389
	let user = '';
	let password = '';
	let loginMsg = '';

	async function login() {
		loginMsg = await invoke('login', { server, port, username: user, password });
	}
</script>

<div>
	<label>
		<p>
			RDP server:[port]
			<input id="login-server" bind:value={server} />:
			<input type="number" id="login-port" bind:value={port} />
		</p>
	</label>
	<label>
		<p>
		Username
		<input id="login-user" bind:value={user} />
		</p>
	</label>
	<label>
		<p>
		Password
		<input type="password" id="login-password" bind:value={password} />
		</p>
	</label>
	<button on:click={login}>login</button>
	<p>{loginMsg}</p>
</div>
