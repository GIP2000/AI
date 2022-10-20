import { useState } from "react";
import type { Tree, RTTree } from "../lib/TreeDT";

const styleSheet = {
    square: (pruned: boolean) => ({
        width: "50px",
        height: "50px",
        backgroundColor: pruned ? "red" : "black",

    }),
    circle: (pruned: boolean) => ({
        width: "50px",
        height: "50px",
        borderRadius: "50%",
        backgroundColor: pruned ? "red" : "black",
    }),
    row: {
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        gap: "15px",
    },
    col: {
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        flexDirection: "column" as 'column',
    }
}

const TreeEl: React.FC<{
    level: Tree<RTTree>
}> = ({ level }) => {
    const [showChildren, setShowChildren] = useState(false);
    return (
        <div>
            <div
                onClick={() => setShowChildren(p => !p)}
                style={styleSheet.col}>
                <div style={level.val.is_max ? styleSheet.circle(level.val.pruned) : styleSheet.square(level.val.pruned)} />
                <div style={{ border: "1px solid black", margin: "15px" }}>
                    [{level.val.alpha},{level.val.beta}] --- {level.val.h_val}
                </div>
            </div>
            {
                showChildren &&
                <div style={styleSheet.row}>
                    {level.next.map((tree) => <TreeEl level={tree} />)}
                </div>
            }
        </div>
    );
}



export default TreeEl;
