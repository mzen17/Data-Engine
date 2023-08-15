<script lang="ts">
    import { GraphNode, LabelNode, SplitNode, FilterNode } from "./Nodes";

    export let node: GraphNode | null = null;
    export let nodes: GraphNode[];
    export let selectNode: Function;
    export let appendNode: Function;
    export let removeNode: Function;

    let addmode = false;
    let select = "";

    function addNode() {
        addmode=false;
        let newNode: GraphNode;
        console.log("Adding...")
        if (select == "split") {
            newNode = new SplitNode([]);
        }
        else if (select == "label") {
            newNode = new LabelNode(["yes"]);
            console.log("Adding label")
        }
        else if(select == "filter") {
            console.log("Adding filter")
            newNode = new FilterNode([]);
        }
        else {
            console.log("Unknown node type: " + select);
            return;
        }
        appendNode(newNode);
    }
    
    const remove = (int: number) => {
        removeNode(int);
    }



</script>
<div class=" h-full ml-auto w-64 flex flex-col">
    <form class = "bg-gray-400 p-4 flex h-[36rem] flex-col rounded-md" >
        <h1 class="text-white text-lg">Properties</h1>
        {#if node != null}
            <h2 class="text-md" title="Name of node">{node.node_name} Node</h2>
            <div title="Start Batch Index. Affects all images starting at this number. Set to disable to affect all before." class="text-md flex items-center h-8">
                <p class="w-8 mr-2">Start:</p>
                <input type="number" placeholder={node.affectRange[0].toString()} class="border-2 ml-2 border-gray-800 w-16 text-sm rounded-md  "/>
            </div>
            <div title="End Batch Ended. Stops affecting all images starting at this number. Set to 0 to affect all after." class="text-md flex items-center h-8">
                <p class="w-8 mr-2">End:</p>
                <input type="number" placeholder={node.affectRange[1].toString()} class="border-2 ml-2 border-gray-800 w-16 text-sm rounded-md  "/>
            </div>
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold w-36 py-2 px-4 mt-auto rounded">Apply</button>    
        {/if}
    </form>
    <div class="w-64 bg-gray-200 flex-grow p-4 rounded-md flex flex-col">
        <p>Layers</p>
        <div class = " w-52 h-64 overflow-scroll bg-gray-100 mb-2">
            {#each nodes as indnode, i}
            <div class="flex">
                <button class={"w-[90%] bg-green-200 transition-all text-center rounded-sm" + ((node!=null && indnode.node_name == node.node_name) ? " bg-green-400 border-green-300":" hover:bg-green-300" )} on:click={() => selectNode(i)}>
                    <p>{indnode.node_name}</p>
                </button>
                <button class={"flex-grow bg-red-300 transition-all text-sm text-center"} on:click={() => remove(i)}>
                    <p>X</p>
                </button>
            </div>
            {/each}
        </div>
    <div class="flex">
        {#if addmode}
            <form class=" flex justify-center items-center bg-black bg-opacity-80">
                <select class="border-2 ml-2 border-gray-800 w-16 text-sm rounded-md  " bind:value={select}>
                    <option value="split">Split</option>
                    <option value="label">Label</option>
                    <option value="filter">Filter</option>
                </select>
                <button class="bg-green-400 w-16 text-sm rounded-md mr-4" on:click={() => {addNode()}}>Add</button>
            </form>
        {:else}
            <button class="bg-green-400 w-16 text-sm rounded-md mr-4" on:click={() => {addmode=true}}>Add</button>
        {/if}
    </div>
    
    </div>
</div>