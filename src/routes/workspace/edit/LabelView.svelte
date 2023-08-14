<script lang="ts">
    export let imgSrc: string;
    export let save: Function
    export let next: Function;
    export let prelabel: string | null = null;

    let prebuild_labels: string[] = ["yes","no"]
    let label: string = "";

    import {afterUpdate} from "svelte";
    afterUpdate (() => {
        if (prelabel != null && label == "") {
            label=prelabel;
        }
        console.log("Prelabel: ", prelabel)

    });

    const setLabel = (target: string) => {
        label = target;
        console.log(nextOnClick);
        if(nextOnClick) {
            submitLabel();
        }
    }

    let addMode = false;
    let newLabel = "";
    let nextOnClick = false;

    const addLabel = (custom_label: string) => {
        if (custom_label == "") {
            return;
        }
        prebuild_labels = [...prebuild_labels, custom_label];
    }

    const submitLabel = () => {
        save(label);
        next(1);
        label="";
    }
    const baksubmitLabel = () => {
        save(label);
        next(-1);
        label="";
    }

</script>

<div class="flex">
    <div>
        <img class = " max-w-[500px] mr-2 h-auto" src={imgSrc} alt="Edit"/>
    </div>
    <div class="flex-col flex">
        <button id="forward" on:click={()=>{submitLabel();}} class="bg-green-400 h-8 rounded-md w-32 p-1 mb-2">Next</button>
        <button id="bak" on:click={()=>{baksubmitLabel()}} class="bg-green-400 h-8 rounded-md w-32 p-1 mb-2">Backward</button>
        <form class="mb-16" on:submit={() => {submitLabel()}}>
            <input type="text" bind:value={label} placeholder="Label:" class="border-2 border-gray-300 rounded-md p-1 w-32"/>
            <p>Next on enter or click</p>
            <input type="checkbox" bind:checked={nextOnClick} class="border-2 border-gray-300 rounded-md p-1 w-32"/>
        </form>
        <div class="flex flex-col mb-4">
        <p>Custom Labels</p>
            {#each prebuild_labels as prebuild_label}
                <button class={"bg-blue-400 h-8 rounded-md w-32 p-1 mb-2 transition-all " + ((prebuild_label == label)? "bg-blue-600":" hover:bg-blue-500")} on:click={()=>{setLabel(prebuild_label)}}>{prebuild_label}</button>
            {/each}
        </div>
        <button class="bg-blue-800 h-8 text-white rounded-md w-32 p-1 mb-2" on:click={()=>{addMode=true}}>Add prebuilt label.</button>
        {#if addMode}
        <div>
            <input type="text" bind:value={newLabel} placeholder="New Label:" class="border-2 border-gray-300 rounded-md p-1 w-32"/>
            <button class="bg-blue-900 h-8 text-white rounded-md w-32 p-1 mb-2" on:click={()=>{addLabel(newLabel);addMode=false}}>Add</button>
        </div>
        {/if}
    </div>
</div>