<script>
	import { onMount } from "svelte";

	onMount(() => {
		const info = localStorage.getItem("sxbp_info");
		if (info) {
			console.log("info", info);
			const infoObj = JSON.parse(info);
		}

		if (localStorage.getItem("current_project") != null) {
			console.log("current_project", localStorage.getItem("current_project"));
			window.location.href = "/workspace";
		}
	});

	const createNewProject = () => {
		window.location.href = "/new";
	};
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section class = "w-screen h-screen flex flex-col justify-center items-center ">
		<h1	class="text-xl block mb-4">
			Welcome to the SX Data Engine!
		</h1>
		<div>
			<p class = "text-md text-gray-400 mb-2 inline-block">Create a new project.</p>
			<button class="w-8 h-8 transition-all hover:bg-green-200 bg-green-400 text-sm rounded-lg" on:click={() => createNewProject()}>+</button>
		</div>
		<div>
			<p class = "text-md text-gray-400 mb-2 inline-block">Load previous Project.</p>
			<button class="w-8 h-8 transition-all hover:bg-green-200 bg-green-400 text-sm rounded-lg" on:click={() => createNewProject()}></button>
			<button class="w-8 h-8 transition-all hover:bg-green-200 bg-green-400 text-sm rounded-lg" on:click={() => createNewProject()}>+</button>

		</div>
</section>