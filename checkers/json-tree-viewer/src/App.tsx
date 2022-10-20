import Tree from "./components/Tree";
import data from "./data/tree.json";
function App() {
    return (
        <>
            <div>Json Tree Viewer</div>
            <Tree
                //@ts-ignore
                level={data}
            />
        </>

    );
}

export default App;
