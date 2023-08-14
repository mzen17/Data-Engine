class GraphNode {
    node_name: string
    description: string
    affectRange: [number, number]
    data: string

    constructor(node_name: string, description: string, affectRange: [number, number], data = "{}"){
        this.node_name = node_name;
        this.description = description;
        this.affectRange = affectRange;
        this.data = data;
    }

    serialize() {
        return JSON.stringify({
            node_name: this.node_name,
            description: this.description,
            affectRange: this.affectRange,
            data: this.data
        });
    }

    // Static method to create an instance from serialized JSON data
    static deserialize(jsonData: any) {
        const data = JSON.parse(jsonData);
        return new GraphNode(data.node_name, data.description, data.affectRange, data.data);
    }
    
}

class SplitNode extends GraphNode{
    indEdit: [[number[], number[]]][]

    constructor(indEdit: [[number[], number[]]][]){
        super("Split", "Split the image into multiple parts. Useful for processing mangas, comics, etc.", [0, 0])
        this.indEdit = indEdit;
        console.log(indEdit)
    }

    serialize() {
        const baseData = super.serialize();
        return JSON.stringify({
            ...JSON.parse(baseData),
            indEdit: this.indEdit
        });
    }

    // Static method to create an instance from serialized JSON data
    static deserialize(jsonData: any) {
        const data = JSON.parse(jsonData);
        return new SplitNode(data.indEdit);
    }
}

class LabelNode extends GraphNode{
    indEdit: string[]

    constructor(indEdit: string[]){
        super("Label", "Label the images. Useful for hand labeling the data", [0, 0])
        this.indEdit = indEdit;
    }

    serialize() {
        const baseData = super.serialize();
        return JSON.stringify({
            ...JSON.parse(baseData),
            indEdit: this.indEdit
        });
    }

    // Static method to create an instance from serialized JSON data
    static deserialize(jsonData: any) {
        const data = JSON.parse(jsonData);
        return new LabelNode(data.indEdit);
    }
}

export { GraphNode, SplitNode, LabelNode };
