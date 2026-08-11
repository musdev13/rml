#!/usr/bin/env bash

set -euo pipefail

# ============================================================
# CONFIG
# ============================================================

GITHUB_REPO="musdev13/rml"
GITHUB_REMOTE="github"
GITLAB_REMOTE="gitlab"

LINUX_BINARY="./target/release/rml"
WINDOWS_BINARY="./target/x86_64-pc-windows-gnu/release/rml.exe"

GITHUB_TEMPLATE="./templates/github-release.md"
GITLAB_TEMPLATE="./templates/gitlab-release.md"

# ============================================================
# HELP
# ============================================================

usage() {
    echo "Usage:"
    echo
    echo "  $0 <rml_version> <rmlib_version> <musutils_version> [options]"
    echo
    echo "Options:"
    echo "  --change <text>              Add release change"
    echo "  --image <url> <height>       Add image"
    echo
    echo "Example:"
    echo
    echo "  $0 1.4.0 0.8.2 0.3.1 \\"
    echo "      --change 'Added NeoForge support' \\"
    echo "      --change 'Fixed Minecraft launch' \\"
    echo "      --image 'https://example.com/image.png' 150 \\"
    echo "      --image 'https://example.com/other.png' 200"
    echo
}

# ============================================================
# ARGUMENTS
# ============================================================

if [ "$#" -lt 3 ]; then
    usage
    exit 1
fi

rml_version="$1"
rmlib_version="$2"
musutils_version="$3"

shift 3

changes=()
images=()

while [ "$#" -gt 0 ]; do
    case "$1" in
        --change)
            if [ "$#" -lt 2 ]; then
                echo "Error: --change requires text"
                exit 1
            fi

            changes+=("$2")
            shift 2
            ;;

        --image)
            if [ "$#" -lt 3 ]; then
                echo "Error: --image requires URL and height"
                exit 1
            fi

            images+=("$2|$3")
            shift 3
            ;;

        -h|--help)
            usage
            exit 0
            ;;

        *)
            echo "Error: unknown argument: $1"
            echo
            usage
            exit 1
            ;;
    esac
done

# ============================================================
# VERSION / TAG
# ============================================================

tag="v${rml_version}-${rmlib_version}-${musutils_version}"

title="v${rml_version} (rmlib: ${rmlib_version}) (musutils: ${musutils_version})"

echo
echo "========================================"
echo " Release"
echo "========================================"
echo " RML:       ${rml_version}"
echo " rmlib:     ${rmlib_version}"
echo " musutils:  ${musutils_version}"
echo " Tag:       ${tag}"
echo "========================================"
echo

# ============================================================
# COMPILING
# ============================================================

cargo build --release
cargo build --release --target x86_64-pc-windows-gnu

# ============================================================
# CHECK FILES
# ============================================================

if [ ! -f "$LINUX_BINARY" ]; then
    echo "Error: Linux binary not found:"
    echo "  $LINUX_BINARY"
    exit 1
fi

if [ ! -f "$WINDOWS_BINARY" ]; then
    echo "Error: Windows binary not found:"
    echo "  $WINDOWS_BINARY"
    exit 1
fi

if [ ! -f "$GITHUB_TEMPLATE" ]; then
    echo "Error: GitHub template not found:"
    echo "  $GITHUB_TEMPLATE"
    exit 1
fi

if [ ! -f "$GITLAB_TEMPLATE" ]; then
    echo "Error: GitLab template not found:"
    echo "  $GITLAB_TEMPLATE"
    exit 1
fi

# ============================================================
# TEMP FILES
# ============================================================

tmp_dir="$(mktemp -d)"

github_notes="${tmp_dir}/github.md"
gitlab_notes="${tmp_dir}/gitlab.md"
gitlab_assets="${tmp_dir}/assets.json"

cleanup() {
    rm -rf "$tmp_dir"
}

trap cleanup EXIT

# ============================================================
# TEMPLATE PROCESSOR
# ============================================================

render_template() {
    local template="$1"
    local output="$2"
    local mode="$3"

    python3 - "$template" "$output" "$mode" \
        "${changes[@]}" \
        -- \
        "${images[@]}" <<'PY'
import sys
import re

template_path = sys.argv[1]
output_path = sys.argv[2]
mode = sys.argv[3]

args = sys.argv[4:]

changes = []
images = []

separator = args.index("--")

changes = args[:separator]
image_args = args[separator + 1:]

for i in range(0, len(image_args), 1):
    value = image_args[i]

    if "|" not in value:
        continue

    url, height = value.split("|", 1)
    images.append((url, height))


with open(template_path, "r", encoding="utf-8") as f:
    template = f.read()


def replace_changes(match):
    item_template = match.group(1)

    result = []

    for change in changes:
        result.append(
            item_template.replace("{% m_text %}", change)
        )

    return "\n".join(result)


def replace_images(match):
    item_template = match.group(1)

    result = []

    for url, height in images:
        item = item_template
        item = item.replace("{% i_url %}", url)
        item = item.replace("{% i_height %}", height)
        result.append(item)

    return "\n".join(result)


# m_el:
#
# {% m_el "..." %}
#
# The contents inside the quotes become the item template.
#
# The parser intentionally allows escaped quotes.
#
template = re.sub(
    r'\{% m_el "(.*?)" %\}',
    replace_changes,
    template,
    flags=re.DOTALL
)


# i_el:
#
# {% i_el "..." %}
#
template = re.sub(
    r'\{% i_el "(.*?)" %\}',
    replace_images,
    template,
    flags=re.DOTALL
)


with open(output_path, "w", encoding="utf-8") as f:
    f.write(template)
PY
}

# ============================================================
# GENERATE GITHUB RELEASE NOTES
# ============================================================

echo "Generating GitHub release notes..."

render_template \
    "$GITHUB_TEMPLATE" \
    "$github_notes" \
    "github"

# ============================================================
# SHOW GITHUB NOTES
# ============================================================

echo
echo "GitHub release notes:"
echo "----------------------------------------"
cat "$github_notes"
echo "----------------------------------------"
echo

# ============================================================
# CREATE GITHUB RELEASE
# ============================================================

echo "Creating GitHub release..."

gh release create "$tag" \
    "$LINUX_BINARY" \
    "$WINDOWS_BINARY" \
    --repo "$GITHUB_REPO" \
    --title "$title" \
    --notes-file "$github_notes"

echo
echo "GitHub release created."
echo

# ============================================================
# FETCH GITHUB TAG
# ============================================================

echo "Fetching GitHub tags..."

git fetch "$GITHUB_REMOTE" --tags

echo "GitHub tag fetched."
echo

# ============================================================
# PUSH TAG TO GITLAB
# ============================================================

echo "Pushing ${tag} to GitLab..."

git push "$GITLAB_REMOTE" "$tag"

echo "GitLab tag pushed."
echo

# ============================================================
# GITHUB ASSET URLS
# ============================================================

linux_name="$(basename "$LINUX_BINARY")"
windows_name="$(basename "$WINDOWS_BINARY")"

github_release_url="https://github.com/${GITHUB_REPO}/releases/download/${tag}"

linux_url="${github_release_url}/${linux_name}"
windows_url="${github_release_url}/${windows_name}"

# ============================================================
# GENERATE GITLAB RELEASE NOTES
# ============================================================

echo "Generating GitLab release notes..."

render_template \
    "$GITLAB_TEMPLATE" \
    "$gitlab_notes" \
    "gitlab"

# ============================================================
# GENERATE GITLAB ASSET LINKS
# ============================================================

cat > "$gitlab_assets" <<EOF
[
  {
    "name": "${linux_name}",
    "url": "${linux_url}",
    "link_type": "other"
  },
  {
    "name": "${windows_name}",
    "url": "${windows_url}",
    "link_type": "other"
  }
]
EOF

# ============================================================
# SHOW GITLAB NOTES
# ============================================================

echo
echo "GitLab release notes:"
echo "----------------------------------------"
cat "$gitlab_notes"
echo "----------------------------------------"
echo

echo "GitLab assets:"
echo "----------------------------------------"
cat "$gitlab_assets"
echo "----------------------------------------"
echo

# ============================================================
# CREATE GITLAB RELEASE
# ============================================================

echo "Creating GitLab release..."

glab release create "$tag" \
    --name "$title" \
    --notes-file "$gitlab_notes" \
    --assets-links "$(cat "$gitlab_assets")"

echo
echo "========================================"
echo " Release successfully created"
echo "========================================"
echo " GitHub: ${tag}"
echo " GitLab: ${tag}"
echo "========================================"
