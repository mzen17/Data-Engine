<script lang="ts">
    import Nav from './Nav.svelte';
    import Himagepane from './panels/himagepane.svelte';
    import Vimagepane from './panels/vimagepane.svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { readBinaryFile } from '@tauri-apps/api/fs';
    import Graph from './graph/Graph.svelte';
    import { GraphNode, SplitNode, LabelNode } from './graph/Nodes';
    import Edit from './edit/Edit.svelte';
    import Review from './review/Review.svelte';

    import {rendergraph} from './render/render';

    let images_url: string[] = []    
    let project_name: string;
    let project_description: string;
    let data_path: string;
    let graph: GraphNode[] = [];


    const project = localStorage.getItem("current_project");
    if(project == null) {
        window.location.href = '/new';
    }else{
        const project_data = localStorage.getItem(project)
        if (project_data != null) {
            const parse = JSON.parse(project_data)
            project_name = parse["name"]
            project_description = parse["description"]
            data_path = parse["data"]
            if (project_data != null && parse["graph"] != null) {
                const graphData = JSON.parse(parse["graph"]);
                graph = graphData.map((item: any) => {
                    switch (item.node_name) {
                        case "Split":
                            return SplitNode.deserialize(JSON.stringify(item));
                        case "Label":
                            return LabelNode.deserialize(JSON.stringify(item));
                        default:
                            return new GraphNode(item.node_name, item.description, item.affectRange, item.data);
                    }
                });
            }
        
        }
    }

    import {onMount} from 'svelte';
    onMount (() => {
        console.log("saving")
        setInterval(cache_save, 30000);
    })


    let current_page = 0;
    const setCurrentPage = (page: number) => {
        current_page = page;
    }


    let selected = 0;
    const setSelectedNode = (node: number) => {
        selected = node;
    }

    const render_graph = async(max: number = 0) => {
        return await rendergraph(img_file_paths, graph, max);
    }


    const setGraph = (newGraph: GraphNode[]) => {
        graph = newGraph;
    }


    let img_file_paths: string[] = [];
    const get_img_paths = async (data_url: string) => {
        console.log("Fetching...")

        let images:string[] = []
        for (const folder of data_url) {
            console.log("GOING TO ", folder)
            try {            
                const res:string[] = await invoke('fetch_files', { dir: folder, extension: 'png' });
                res.forEach((file: string) => {
                    images = [...images, file]
                })     
                const res2:string[] = await invoke('fetch_files', { dir: folder, extension: 'jpg' });
                res2.forEach((file: string) => {
                    images = [...images, file]
                })             
            } catch (error) {
                console.error('Error:', error);
            }
        }
        return images
    }; 


    const render_img_paths = async(uris: string[], maxURIs=-1, start_index=0) => {
        let bloburls:string[] = [];
        for (let i = start_index; i<uris.length && (maxURIs < 0 || i<start_index+maxURIs); i++) {
            const uri = uris[i];
            const contents = await readBinaryFile(uri);

            const blob = new Blob([contents], { type: 'image/jpeg' });
            const url = URL.createObjectURL(blob);
            bloburls = [...bloburls, url]
            images_url = [...images_url, url]
        }
        return bloburls
    }


    const save = (node: GraphNode) => {
        console.log("Node: ", node)
        graph[selected] = node;
    }


    const cache_save = () => {
        console.log("Saving...")
        const project_graph = JSON.stringify(graph)
        const project = {
            "name": project_name,
            "description": project_description,
            "data": data_path,
            "graph": project_graph
        }
        localStorage.setItem(project_name, JSON.stringify(project))
    }

    const global_save = () => {
        const project_graph = JSON.stringify(graph)
        const project_data = localStorage.getItem("current_project")
        const data_path = JSON.parse(project_data!)["data"]
        const project_name = JSON.parse(project_data!)["name"]
        const project_description = JSON.parse(project_data!)["description"]

        const project = {
            "name": project_name,
            "description": project_description,
            "data": data_path,
            "graph": project_graph
        }

        localStorage.setItem(project_name, JSON.stringify(project))
    }


    const load_page = async () => {
        const images = await get_img_paths(data_path);
        img_file_paths = images;
        await render_img_paths(images);
    }


    load_page()
</script>


<svelte:head>
  <title>Workspace</title>
  <meta name="description" content="Preview data." />
</svelte:head>

<section class="w-screen overflow-clip flex flex-col p-4" style="height: calc(100vh - 64px);">
    {#if current_page == 0}
        <h1>Preview</h1>
        <Himagepane images = {images_url} />
    {:else if current_page == 3}
        <h1>Review</h1>
        <Review get_render={render_graph} />
    {:else}
    <div class="flex w-screen " style="height: calc(100vh - 64px);">
        {#if current_page == 1}
            <Vimagepane images = {images_url} />
            <Graph selectedNode = {selected} setSelectedNode={setSelectedNode} nodes={graph} set_nodes={setGraph}/>
        {:else if current_page == 2}
            <Edit batch={images_url} save={save} node={(graph[selected] != null)? graph[selected]: null}/>
        {/if}
    </div>
    {/if}
</section>
<Nav selected={current_page} setCurrent={setCurrentPage} save={cache_save}/>
