<script lang="ts">
    export let get_render: Function;
    import Himagepane from '../panels/himagepane.svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { open } from '@tauri-apps/api/dialog';
    import { appDataDir } from '@tauri-apps/api/path';


    let images: string[] = [];
    let download_path: string;
    let previewAmount: number = 3;
    let loading = false;

    const render =async() => {
        images = (await get_render(previewAmount))[0];
        console.log(images);
    }

    const final_render = async() => {
        if(download_path != "") {
            images = await get_render(0)[1];
        }
        // Invoke
        const result = await invoke('copyfiles_to_location', {images: images, path: download_path});
    }

    const select = async () =>{
    let tmpselected = await open({
        directory: true,
        multiple: false,
        defaultPath: await appDataDir(),
      });

      if (tmpselected !== null) {
        download_path = tmpselected[0];
    }
  }
</script>

<div class = "w-full flex flex-col">
    <div>
        <input class="w-64 rounded-md border-2 border-gray-300" type="number" min="1" max="100" bind:value={previewAmount}/>
        <button class="w-32 rounded-md border-2 border-gray-300" on:click={()=>{render()}}>Preview</button>
        <button class="w-32 rounded-md border-2 border-gray-300" on:click={()=>{select()}}>Export Location</button>
        <button class="w-32 rounded-md border-2 border-gray-300" on:click={()=>{final_render()}}>Render</button>
    </div>
    <Himagepane images={images}/>
</div>