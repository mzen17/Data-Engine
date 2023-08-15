import type { GraphNode, SplitNode, LabelNode } from "../graph/Nodes"
import { invoke } from '@tauri-apps/api/tauri';
import { readBinaryFile } from '@tauri-apps/api/fs';


const rendergraph = async(data_url: string[], nodes: GraphNode[], amount: number, depth: number=0) => {
    // Invoke the render function from the tauri backend and get the URLs
    let files: string[] = [];
    console.log("Starting...")

    let index = 0;
    for (const file of data_url) {
        let nodeInd = 0;
        if(nodes.length > 0) {
            for (const node of nodes) {

                if (node.node_name === 'Split') {

                    const split_node = node as SplitNode;
                    const lines = split_node.indEdit;
                    let formattedLines: number[][] = [];
                    
                    if (lines.length > index && lines[index] != null) {
                        for (const line of lines[index]) {
                            let newline = [line[0][0], line[0][1], line[1][0], line[1][1]];
                            formattedLines.push(newline);
                        }
                    }
                    let url:string[] = await invoke('split_img_render', { imgPath: file, lines: formattedLines, id: index });
                    for (const u of url) {
                        files = [...files, u];
                    }

                }
                nodeInd++;
                if(depth > 0 && nodeInd >= depth) break;
            }
        index++;

        if(amount > 0 && index > amount) break;
        }else{
            files = [...files, file];
        }
    }

    console.log("Pass1")

    let bloburls:string[] = [];
    for (let i = 0; i<files.length; i++) {
        console.log("Reading ", i, "...")
        const uri = files[i];
        const contents = await readBinaryFile(uri);

        const blob = new Blob([contents], { type: 'image/jpeg' });
        const url = URL.createObjectURL(blob);
        bloburls = [...bloburls, url]
    }
    console.log("Finished with: ", bloburls, " | ", files)
    return [bloburls, files];
}
export {rendergraph}