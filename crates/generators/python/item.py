from common import load_minecraft_json, camel_case, generate_enum, generate_enum_property, output

items = []
ids = {}
names = {}
display_names = {}
stack_sizes = {}
durabilities = {}

for item in load_minecraft_json("items.json", "1.16.2"):
    variant = camel_case(item['name'])
    items.append(variant)
    ids[variant] = item['id']
    names[variant] = item['name']
    display_names[variant] = item['displayName']
    stack_sizes[variant] = item['stackSize']

    durability = item.get('durability')
    if durability is None:
        durabilities[variant] = "None"
    else:
        durabilities[variant] = f"Some({durability})"

output_data = generate_enum("Item", items)
output_data += generate_enum_property("Item", "id", "u32", ids, True)
output_data += generate_enum_property("Item", "name", "&str", names, True, "&'static str")
output_data += generate_enum_property("Item", "display_name", "&str", display_names, True, "&'static str")
output_data += generate_enum_property("Item", "stack_size", "u32", stack_sizes)
output_data += generate_enum_property("Item", "durability", "Option<u32>", durabilities)

output("crates/items/src/item.rs", output_data)
