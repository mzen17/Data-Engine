<script lang="ts">
    export let images:string[] = []
</script>

<div class = "w-screen overflow-y-scroll">
    {#each images as image}
        <img class="w-24 h-24 inline-block object-cover m-1 hover:w-64 hover:h-64 transition-all" src={image} alt={image}/>
    {/each}
</div>