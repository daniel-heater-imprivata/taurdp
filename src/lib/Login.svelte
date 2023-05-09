<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { goto } from '$app/navigation';

	let server = '';
	let port = 3389;
	let user = '';
	let password = '';
	let loginMsg = '';
	let errormsg = '';

	async function login() {
		loginMsg = await invoke('login', { server, port, username: user, password })
			.then((response) => {
				console.log(response);
				errormsg = '';
				goto("/view")
			})
			.catch((response) => {
				console.error(response);
				errormsg = response;
			});
	}
</script>

<div>
	<h2>Let's get connected!</h2>
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
	<h3 style="color: red">{errormsg}</h3>
</div>
