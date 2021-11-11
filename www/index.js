import {Sudoku} from "rustokuwasm";


const sud = document.getElementById("sudoku");
let puz = Sudoku.new("...16.87..1.875..38.73..651.5.62173...17..5.473.5..1...7........8.256917.62..7...");
const board_size = Sudoku.board_size();
const hintapplyButton = document.getElementById("hint");
const clearHintButton = document.getElementById("clear");
const message = document.getElementById("message-p");
const createPuzzleButton = document.getElementById("puzzle_submit");


let move;

function addHTMLElement (where, str) {
    let div = document.createElement("div");
    div.innerHTML = str;
    where.append(div.firstChild);
}

function addSquares () {

    let len = 9;
    let num_cells = len * len;
    for (let ind = 0; ind < num_cells; ++ind) {
        let str = "<div class='sudoku-board-cell'>" +
            "<input type='text' pattern='\\d*' novalidate id='sq" +ind+ "' value=''>" +
            "<div id='sq" +ind+ "-cand' class='candidates'>" + "12" + "</div>" +
            "</div>";

        addHTMLElement(sud, str);
        if (ind + 1 % len === 0) {
            let br = document.createElement("br");
            sud.append(br.firstChild);
        }
    }

}


function set_squares() {
    let values = puz.values();
    //sud.classList.add("showCandidates");

    for (let i = 0; i < values.length; ++i) {
        let sq = document.getElementById("sq" + i);
        let sqp = document.getElementById("sq" + i + "-cand");
        let value;
        if (values[i] === 0) {
            value = ""
            let possibils = puz.poss(i);
            let pstr = candidates_string(possibils);

            sqp.innerHTML = pstr;


        } else {
            value = values[i];
            if (puz.is_fixed(i)){
                sq.disabled="true";
            }

            sqp.innerHTML = "";

        }
        sq.value = value;

    }
}

function candidates_string (possibils) {
    let str = "";

    for (let v = 1; v <= board_size; ++v) {
        if (possibils.find((a) => {return a === v})) {
            str += "<div>" + v + "</div>";
        } else {
            str += "<div>&nbsp;</div>";
        }
    }

    return str;
}

function get_hint () {
    move = puz.hint();
    move.update_board();
}

function apply() {
    if (!move) {
        return;
    }
    move.apply(puz);
    updateMessage("");
    set_squares();

}

function updateMessage(string) {
    let message = document.getElementById("message-p");
    message.innerHTML = string;
}

function getSqElement(ind) {
    let sq = document.getElementById("sq"+ind);
    let sqp = document.getElementById("sq"+ind+"-cand");
    return {sq, sqp};
}

function resetHintButton () {
    hintapplyButton.innerHTML = "Hint";
    clearHintButton.hidden = true;

}

function resetMessage() {
    message.innerHTML="";
}

function clearHint () {
    document.querySelectorAll(".candidate--highlight").forEach(element => {
        console.log(element);
        element.classList.remove(["candidate--highlight"]);
    });
    document.querySelectorAll(".candidate--to-remove").forEach(element => {
        console.log(element);
        element.classList.remove(["candidate--to-remove"]);
    });
    resetHintButton();
    resetMessage();
}

hintapplyButton.onclick = () => {
    if (hintapplyButton.innerHTML === "Hint") {
        get_hint();
        hintapplyButton.innerHTML = "Apply";
        clearHintButton.hidden = false;
    } else {
        apply();
        resetHintButton();
    }
}

clearHintButton.onclick = () => {
    clearHint();
}

createPuzzleButton.onclick = () => {
    let inputbox = document.getElementById("puzzle_input");

    let string = inputbox.value;
    console.log(string);
    puz = Sudoku.new(string);
    clearHint();
    set_squares();
    inputbox.innerText = "";

}

addSquares();
set_squares();


