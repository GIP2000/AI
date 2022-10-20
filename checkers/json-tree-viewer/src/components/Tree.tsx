import { useState } from "react";
import type { Tree, RTTree } from "../lib/TreeDT";

const styleSheet = {
    flip: {
        transform: "rotate(180deg) translateY(-75px)"
    },
    row: {
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
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
    console.log(level.val);
    return (
        <div>
            <div
                onClick={() => setShowChildren(p => !p)}
                style={styleSheet.col}>
                <div style={level.val.is_max ? {} : styleSheet.flip}>
                    <Triangle />
                </div>
                <div>
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


const Triangle: React.FC = () => {

    return (
        <svg height="150" width="500" >
            <polygon points="250,60 100,400 400,400" />
            Sorry, your browser does not support inline SVG.
        </svg >
    );

}

export default TreeEl;
