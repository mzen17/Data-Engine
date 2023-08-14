<script lang="ts">    
    export let selectedNode: number;
    export let setSelectedNode: Function;

    import type { GraphNode } from "./Nodes";
    import Properties from "./Properties.svelte";

    export let nodes: GraphNode[]
    export let set_nodes: Function

    const addNode = (node: GraphNode) => {
        const new_nodes = [...nodes, node];
        set_nodes(new_nodes)
    }
    const remoteNodeatIndex = (index: number) => {
        const new_nodes = nodes.slice(0, index).concat(nodes.slice(index + 1));
        set_nodes(new_nodes)
    }
</script>


<div id="Graph" class="ml-2 flex overflow-scroll w-[80vw]">
    <h1 class="text-lg">Graph</h1>
    <div class=" h-full flex justify-center items-center">
        <div title="Start Node" class="w-64 p-4 bg-blue-500 text-center rounded-sm rounded-e-2xl border-2">
            <p>Input</p>
        </div>
        {#each nodes as node, i}
            <button title={node.description} class={"w-64 p-4 bg-green-200 transition-all text-center rounded-sm rounded-e-2xl border-2 border-green-300 " + ((selectedNode == i)?" bg-green-400":"hover:bg-green-300")} on:click={() => {
                setSelectedNode(i);
            }}>
                <p>{node.node_name}</p>
            </button>
        {/each}
        <div title="Output Node" class="w-64 p-4 bg-blue-500 text-center rounded-sm rounded-e-2xl border-2">
            <p>Output</p>
        </div>
    </div>
</div>
<Properties node={(nodes.length > 0) ? nodes[selectedNode] : null} selectNode={setSelectedNode} appendNode={addNode} nodes={nodes} removeNode={remoteNodeatIndex}/>
