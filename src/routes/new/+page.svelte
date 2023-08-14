<script lang="ts" context="module">
  import { open } from '@tauri-apps/api/dialog';
  import { appDataDir } from '@tauri-apps/api/path';

  let selected: any = []; 
  const select = async () =>{
    let tmpselected = await open({
        directory: true,
        multiple: true,
        defaultPath: await appDataDir(),
      });
      if (Array.isArray(tmpselected)) {
        tmpselected.forEach((element: any) => {
          selected.push(element);
        });
      } else if (tmpselected !== null) {
        selected.push(tmpselected);
    }
    console.log(selected)
  }

  let progress = 0;
  let project_name = '';
  let project_description = '';

 function submitForm() {
    console.log("submit")
    const project = {
      name: project_name,
      description: project_description,
      data: selected
    }

    // Turn project to json
    const project_json = JSON.stringify(project);
    localStorage.setItem(project_name, project_json);
    localStorage.setItem("current_project", project_name);
    window.location.href = '/workspace';
  }

  let width1 = 10;
  let width2 = 10;

  let getWidths = () =>{
      let vx = window.innerWidth;
      let w1, w2;
      if(vx < 300) {
        w1 = 100  ;
      } else if(vx < 600) {
        w1 = 200;
      } else if(vx < 1024) {
        w1 = 300;
      } else {
        w1 = 300;
      }
        w2 = vx - w1;
        return [w1, w2]
  }

  width1 = getWidths()[0];
  width2 = getWidths()[1];

  window.addEventListener('resize', function(event) {
    width1 = getWidths()[0];
    width2 = getWidths()[1];
  }, true);


</script>

<svelte:head>
  <title>New Project</title>
  <meta name="description" content="Create a new batch editing project." />
</svelte:head>

<section class="w-screen h-screen flex flex-col">
  <form class="flex-row flex">
    <div class={`flex-col flex h-screen bg-gray-300 p-4`} style={`width:${width1}px`}>
      <h1 class="text-xl block mb-4">Create a new project.</h1>
      
      <h2 class="text-md text-gray-700 mb-2 inline-block">Project Settings.</h2>
      <p class="text-md text-gray-400 mb-2 inline-block">Name</p>

      <input class="p-0  text-black w-64 md:w-32 h-4 bg-gray-200 rounded-md" bind:value={project_name} />
      <p class="text-md text-gray-400 mb-2 inline-block">Description</p>
      <input class="p-2 w-64 h-32 bg-gray-200 rounded-mm text-left flex" bind:value={project_description} />
    </div>

    <div class={`bg-gray-200 p-4 flex`} style={`width:${width2}px`}>
      <p class="text-md text-gray-600 mb-2 inline-block">Data</p>
      <div class="mt-auto flex justify-center items-center">
        <button class="w-32 h-12 rounded-lg bg-green-400 flex justify-center items-center" on:click={() => {select()}}>
          <p>Upload</p>
        </button>
        <div class="ml-2 hidden">
          <p>Uploading...</p>
          <progress id="progressBar" class={`w-64 bg-green-300 ${progress === 1 ? 'hidden' : ''}`} value={progress} max="100"></progress>
        </div>
      </div>
      <div class="ml-auto mt-auto h-16 flex">
        <label for="prev" class="mt-auto w-32 h-8 flex rounded-lg bg-gray-300 justify-center items-center mr-4">
          <p>Preview</p>
          <input id="prev" class="hidden" type="submit" />
        </label>
        <button class="mt-auto w-32 h-8 flex rounded-lg bg-gray-300 justify-center items-center" on:click={() => submitForm()}>
          <p>Create</p>
        </button>
      </div>
    </div>
  </form>
</section>
