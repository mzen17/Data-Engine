<script lang="ts">
    export let selected: number;
    export let setCurrent: Function;
    export let save: Function;

    const buttons = ["Preview", "Graph", "Edit", "Review"];

    const swap = (switchTo: string) => {
        selected = buttons.indexOf(switchTo);
        console.log(selected);
        setCurrent(selected);
    };
</script>


<footer class="h-16 w-[100vw] bg-gray-200 flex justify-center items-center">
    <button class = "p-4 w-32 border-x-2 border-x-gray-800" on:click={()=>window.location.href="/new"}>Home</button>
    <div class = "w-[80%] flex justify-center items-center">
        {#each buttons as button}
            <button class = {"p-4 w-32 border-x-2 border-x-gray-800" + (button == buttons[selected] ? " border-b-8 border-b-green-300":"")} on:click={()=>swap(button)}>{button}</button>
        {/each}
    </div>
    <button class = "p-4 w-32 border-x-2 border-x-gray-800" on:click={()=>save()}>Save</button>
</footer>