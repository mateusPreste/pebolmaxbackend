#!/usr/bin/env python3
"""
figma_components.py – interactive Figma → YAML extractor
Now saves *all* custom typography in a `typography_styles` block
at the beginning of each YAML file.

Dependencies:
  pip install requests pyyaml python-dotenv
"""

#Chhp98SWqMIjEfO5haHmoO

import sys, requests, yaml, os
from pathlib import Path
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()

# ── CONFIG ────────────────────────────────────────────────────────────────
API_TOKEN = os.getenv("FIGMA_API_TOKEN")   # Load token from environment variable
API       = "https://api.figma.com/v1"
HEADERS   = {"X-Figma-Token": API_TOKEN}

# ── GENERIC HELPERS ───────────────────────────────────────────────────────
def get(url, **params):
    r = requests.get(url, headers=HEADERS, params=params, timeout=30)
    r.raise_for_status()
    return r.json()

def list_pages(file_key):
    data = get(f"{API}/files/{file_key}", depth=1)
    return [n for n in data["document"]["children"] if n["type"] == "CANVAS"]

def walk(node, wanted, hits):
    if node["type"] in wanted:
        hits.append(node)
    for child in node.get("children", []):
        walk(child, wanted, hits)

def components_on_page(file_key, page_id):
    doc = get(f"{API}/files/{file_key}/nodes", ids=page_id)["nodes"][page_id]["document"]
    hits = []
    walk(doc, {"COMPONENT", "COMPONENT_SET"}, hits)
    return hits

# ── TEXT‑STYLE REGISTRY ───────────────────────────────────────────────────
def fetch_typography_styles(file_key):
    """Return tuple (style_map, typography_yaml_mapping)."""
    meta = get(f"{API}/files/{file_key}/styles")["meta"]["styles"]
    print(meta)
    text_styles = [s for s in meta if s["style_type"] == "TEXT"]


    # Gather all node_ids that hold the actual style definition
    node_ids = [s["node_id"] for s in text_styles]
    style_nodes = {}
    # Figma API allows many ids, but chunk to 99 to be safe
    for i in range(0, len(node_ids), 99):
        ids_param = ",".join(node_ids[i:i+99])
        part = get(f"{API}/files/{file_key}/nodes", ids=ids_param)["nodes"]
        style_nodes.update(part)

    typography_yaml = {}
    style_map = {}          # key → {name, description}

    for s in text_styles:
        key      = s["key"]           # used by TEXT nodes (styleId)
        name     = s["name"]
        desc     = s.get("description", "")
        node_id  = s["node_id"]
        node     = style_nodes[node_id]["document"]
        style    = node.get("style", {})  # actual typography values

        # Build YAML record
        typography_yaml[name] = {
            "key":         key,
            "fontFamily":  style.get("fontFamily"),
            "fontWeight":  style.get("fontWeight"),
            "fontStyle":   "italic" if style.get("italic") else "normal",
            "fontSize":    style.get("fontSize"),
            "lineHeight":  style.get("lineHeightPx") or style.get("lineHeightPercent"),
            "letterSpacing": style.get("letterSpacing"),
            "description": desc,
        }

        style_map[key] = {"name": name, "description": desc}

    return style_map, typography_yaml

# ── NODE → YAML TRANSFORM ────────────────────────────────────────────────
AUTO_KEYS = [
    "layoutMode", "primaryAxisSizingMode", "counterAxisSizingMode",
    "primaryAxisAlignItems", "counterAxisAlignItems",
    "itemSpacing",
    "paddingLeft", "paddingRight", "paddingTop", "paddingBottom",
    "layoutGrow", "layoutAlign", "layoutPositioning"
]

def simplify_node(node, style_map):
    out = {
        "id":   node["id"],
        "name": node["name"],
        "type": node["type"],
        "box":  node.get("absoluteBoundingBox", {})
    }

    # Auto‑Layout
    auto = {k: node[k] for k in AUTO_KEYS if k in node}
    if auto:
        out["auto_layout"] = auto

    # TEXT specifics
    if node["type"] == "TEXT":
        shared_name = style_map.get(node.get("styleId"), {}).get("name")
        out["text_style"] = {
            "shared": shared_name,           # e.g. "Heading/H1"
            "local":  node.get("style", {})  # overrides only
        }
        out["characters"] = node.get("characters", "")

    # Recurse
    if "children" in node:
        out["children"] = [simplify_node(c, style_map) for c in node["children"]]

    return out

def render_yaml(component_node, style_map, typography_yaml, path):
    """Write YAML with typography_styles at top."""
    doc = {
        "typography_styles": typography_yaml,
        "component": simplify_node(component_node, style_map)
    }
    with open(path, "w", encoding="utf-8") as f:
        yaml.safe_dump(doc, f, sort_keys=False, allow_unicode=True)

def choose(items, label, single=True):
    print(f"\nChoose {label}:")
    for i, it in enumerate(items, 1):
        print(f"  {i:>2}. {it['name']}")
    choice = input("Enter number (or 'a' for all): ").strip().lower()
    if choice in {"a", "all"} and not single:
        return items
    try:
        return [items[int(choice) - 1]]
    except (ValueError, IndexError):
        sys.exit("✖ Invalid choice, exiting.")

# ── MAIN FLOW ─────────────────────────────────────────────────────────────
def main():
    if not API_TOKEN:
        sys.exit("✖ Please set your FIGMA_API_TOKEN in the .env file.")

    file_key = sys.argv[1] if len(sys.argv) > 1 else input("Enter Figma FILE_KEY: ").strip()
    pages = list_pages(file_key)
    if not pages:
        sys.exit("✖ No pages found.")

    page = choose(pages, "a page")[0]
    comps = components_on_page(file_key, page["id"])
    if not comps:
        sys.exit("✖ No components on that page.")
    chosen = choose(comps, "component(s)", single=False)

    # 1) Download *all* typography styles once
    style_map, typography_yaml = fetch_typography_styles(file_key)

    # 2) Dump YAML for each chosen component
    out_dir = Path("figma_yaml"); out_dir.mkdir(exist_ok=True)
    for node in chosen:
        slug = node["name"].replace("/", "_").replace(" ", "_")
        path = out_dir / f"{slug}_{node['id'].replace(':', '_')}.yaml"
        render_yaml(node, style_map, typography_yaml, path)
        print(f"✓ Wrote {path}")

    print("Done!  Typography styles now appear at the beginning of each YAML 🎉")

if __name__ == "__main__":
    main()
