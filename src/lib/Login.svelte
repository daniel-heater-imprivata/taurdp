<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let server = '';
	let user = '';
	let password = '';
	let loginMsg = '';

	async function login() {
		loginMsg = await invoke('login', { server, username: user, password });
	}
</script>

<div>
	<label>
		<p>
			RDP server[:port]
			<input id="login-input" bind:value={server} />
		</p>
	</label>
	<label>
		<p>
		Username
		<input id="login-input" bind:value={user} />
		</p>
	</label>
	<label>
		<p>
		Password
		<input type="password" login-input bind:value={password} />
		</p>
	</label>
	<button on:click={login}>login</button>
	<p>{loginMsg}</p>
</div>
