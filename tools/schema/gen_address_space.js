let _ = require("lodash");
let fs = require("fs");
let xml2js = require("xml2js");

let settings = require("./settings");
let util = require("./util");

let nodeset = require("./nodeset");

const MAX_NODES_PER_FILE = 100;

let trace = false;

// THIS file will generate the address space

let node_set =
    [
        {
            name: "Opc.Ua.NodeSet2.Part3.xml", module: "nodeset_3"
        },
        {
            name: "Opc.Ua.NodeSet2.Part4.xml", module: "nodeset_4"
        },
        {
            name: "Opc.Ua.NodeSet2.Part5.xml", module: "nodeset_5"
        },
        {
            name: "Opc.Ua.NodeSet2.Part8.xml", module: "nodeset_8"
        },
        {
            name: "Opc.Ua.NodeSet2.Part9.xml", module: "nodeset_9"
        },
        {
            name: "Opc.Ua.NodeSet2.Part10.xml", module: "nodeset_10"
        },
        {
            name: "Opc.Ua.NodeSet2.Part11.xml", module: "nodeset_11"
        },
        {
            name: "Opc.Ua.NodeSet2.Part12.xml", module: "nodeset_12"
        },
        {
            name: "Opc.Ua.NodeSet2.Part13.xml", module: "nodeset_13"
        },
        {
            name: "Opc.Ua.NodeSet2.Part14.xml", module: "nodeset_14"
        },
        {
            name: "Opc.Ua.NodeSet2.Part999.xml", module: "nodeset_999"
        }
    ];

///////////////////////////////////////////////////////////////////////////////
// Parse all XML inputs into data and place it on the node sets above

let parser = new xml2js.Parser();

let config = {
    destination_dir: settings.rs_address_space_dir,
    max_nodes_per_file: MAX_NODES_PER_FILE,
    autogenerated_by: "tools/schema/gen_address_space.js",
    trace: trace
}

let modules = [];
_.each(node_set, model => {
    let data = fs.readFileSync(`${settings.schema_dir}/${model.name}`);
    parser.parseString(data, (err, result) => {
        model.data = result;
        console.log(`Generating code for module ${model.module}`);
        let node_set_modules = nodeset.generate(model.name, model.data, model.module, config);
        modules.push(...node_set_modules)
    });
});
console.log(`modules = ${modules}`);

///////////////////////////////////////////////////////////////////////////////
// Create the mod.rs

let mod_contents = `// This file was autogenerated by ${config.autogenerated_by}
// DO NOT EDIT THIS FILE

use crate::address_space::types::AddressSpace;

`;

// use each part
_.each(modules, module => {
    mod_contents += `mod ${module};\n`
});
mod_contents += `\n`;

// in a populate_address_space method
mod_contents += `/// Populates the address space with all defined node sets
pub fn populate_address_space(address_space: &mut AddressSpace) {\n`;

_.each(modules, module => {
    mod_contents += `    ${module}::populate_address_space(address_space);\n`
});

mod_contents += `}\n`;

util.write_to_file(`${settings.rs_address_space_dir}/mod.rs`, mod_contents);

