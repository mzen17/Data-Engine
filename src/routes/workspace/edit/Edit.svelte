<script lang="ts">
    import Canvas from "./Canvas.svelte"
    import LabelView from "./LabelView.svelte"
    import { SplitNode, type GraphNode, LabelNode } from "../graph/Nodes";

    export let node: GraphNode | null = null
    export let batch: string[]
    export let save: Function

    let index = 0;
    let remaining = batch.length - index;
    let transform:[[number[], number[]]][] = []
    let labels: string[] = []
    let imgSrc = batch[index]

    const node_save = () => {
        if (node == null) {
            console.log("Node is null");
            return;
        }
        if (node.node_name == "Split") {
            const split_node = new SplitNode(transform);
            console.log("Attempt to save node: ", split_node)
            save(split_node);
        }
        if (node.node_name == "Label") {
            const label_node = new LabelNode(labels);
            console.log("Attempt to save node: ", label_node)
            save(label_node);
        }
    }

    const next = (forward: number) => {
        if (index + forward < 0 || index + forward >= batch.length) {
            return;
        }
        index += forward;
        imgSrc = batch[index];
    }

    const saveLines = (lines: [[number[], number[]]]) => {
        transform[index] = lines;
    }

    const saveLabels = (label: string) => {
        console.log("Saving label...", label)
        labels[index] = label;
    }

    const node_type = ((node == null)? "null" : node.node_name);
    if(node_type=="Split") {
        const split_node = node as SplitNode;
        if (split_node.indEdit != null) {
            transform = split_node.indEdit;
        }
    }
    console.log(node_type)

</script>

<div class="flex">
    {#if node != null && node_type == "Split"}
    <Canvas lines={(transform[index] == null) ? [] : transform[index]} next={next} imgSrc={imgSrc} save_lines={saveLines}/>
    {:else if node_type == "Label"}
        <LabelView imgSrc={imgSrc} save={saveLabels} next={next} prelabel={labels[index]}/>
    {/if}
    <div class = "flex flex-col ml-2">
        <h1>Batch#: {index+1}</h1>
        <h2>Remaining: {remaining}</h2>
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click={() => node_save()}>Save</button>
    </div>
</div>