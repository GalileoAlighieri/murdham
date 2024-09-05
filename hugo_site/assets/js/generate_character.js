import * as Data from "./data.js"

function random_array_element(array) {
    let idx = Math.floor(Math.random() * array.length);
    return array[idx];
}

function ability_arrays() {
    let min = 5;
    let max = 14;
    let values = []
    for (let a = min; a <= max; ++a) {
        for (let b = min; b <= max; ++b) {
            for (let c = min; c <= max; ++c) {
                if (a + b + c == 24) {
                    values.push([a, b, c]);
                }
            }
        }
    }
    return values;
}

function random_abilities() {
    const arrays = ability_arrays();
    return random_array_element(arrays);
}

function random_money() {
    return Math.floor(Math.random() * 8 + 1);
}

function random_age() {
    return 5 + 10 * Math.floor(Math.random() * 4 + 1) + Math.floor(Math.random() * 10 + 1);
}

function generate_character() {
    let [str, agi, wit] = random_abilities();
    let background = random_array_element(Data.backgrounds);
    let [weapon, money1] = random_array_element(Data.weapons);
    let [item, money2] = random_array_element(Data.items);
    let money3 = random_money();

    let age = random_age();
    let gender = random_array_element(Data.genders);
    let goal = random_array_element(Data.goals);
    let appearance = random_array_element(Data.appearances);
    let personality = random_array_element(Data.personalities)

    let first_name = undefined;
    let background_name = undefined;
    if (gender == "Male") {
        first_name = random_array_element(Data.masculine_first_names);
        background_name = background.masculine_name;
    }
    else {
        first_name = random_array_element(Data.feminine_first_names);
        background_name = background.feminine_name;
    }
    let last_name = random_array_element(Data.last_names);

    let assets = [weapon, item];
    for (let i = 0; i < background.assets.length; i++) {
        assets.push(background.assets[i]);
    }
    for (let i = 0; i < background.sorcerous_scrolls; i++) {
        assets.push(random_array_element(Data.sorcerous_scrolls));
    }
    for (let i = 0; i < background.sacred_scrolls; i++) {
        assets.push(random_array_element(Data.sacred_scrolls));
    }

    let assets_map = {};
    for (asset of assets) {
        if (asset in assets_map) {
            assets_map[asset]++;
        }
        else {
            assets_map[asset] = 1;
        }
    }


    let assets_str = "";
    for (const [asset, count] of Object.entries(assets_map)) {
        if (count > 1) {
            assets_str += count + "× ";
        }
        assets_str += asset + ", ";
    }


    let money = money1 + money2 + money3 + background.money;

    document.getElementById("name").innerHTML = first_name + " " + last_name + " the " + background_name;
    document.getElementById("description").innerHTML =
        gender +", " + age + " years old. " +
        background.description +
        " You have abandoned your previous life because " + goal +
        " " + appearance +
        " " + personality;
    document.getElementById("str").innerHTML = str + background.abilities[0];
    document.getElementById("agi").innerHTML = agi + background.abilities[1];
    document.getElementById("wit").innerHTML = wit + background.abilities[2];
    document.getElementById("mana").innerHTML = background.mana;
    if (background.mana == 0) {
        document.getElementById("mana-display").style.display = 'none';
    }
    else {
        document.getElementById("mana-display").style.display = 'inline';
    }

    let skill_content = "";
    for (let [skill_name, skill_descr] of Object.entries(background.skills)) {
        skill_content += `<div><strong>${skill_name}:</strong> ${skill_descr}</div>\n`
    }

    document.getElementById("skills").innerHTML = skill_content;
    document.getElementById("assets").innerHTML = assets_str + money + "ʂ";
}

document.getElementById("chargen_button").onclick = generate_character;
generate_character();
