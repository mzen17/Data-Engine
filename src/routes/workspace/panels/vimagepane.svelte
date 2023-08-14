<script lang="ts">
    export let images: string[] = []
</script>
<div class = "flex flex-col w-16 bg-gray-100 overflow-scroll items-center">
    {#each images as image}
        <img class="w-12 h-12 object-cover m-1 hover:w-16 hover:h-16 transition-all" src={image} alt={image}/>
    {/each}
</div>