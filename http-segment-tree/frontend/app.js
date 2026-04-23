const BASE_URL = "http://localhost:8000";

async function createTree() {
  const input = document.getElementById("inputArray").value;

  const array = input.split(",").map(Number);

  const res = await fetch(`${BASE_URL}/segment-tree`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ input: array }),
  });

  const data = await res.json();
  render(data);
  loadTree();
}

async function loadTree() {
  const res = await fetch(`${BASE_URL}/segment-tree`);
  const data = await res.json();

  render(data);
}

async function updateTree() {
  const idx = Number(document.getElementById("updateIndex").value);
  const value = Number(document.getElementById("updateValue").value);

  const res = await fetch(`${BASE_URL}/segment-tree`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ idx, value }),
  });

  const data = await res.json();
  render(data);
  loadTree();
}

async function queryTree() {
  const left = document.getElementById("queryLeft").value;
  const right = document.getElementById("queryRight").value;

  const res = await fetch(
    `${BASE_URL}/segment-tree/query?left=${left}&right=${right}`
  );

  const data = await res.json();

  document.getElementById("queryResult").innerText =
    "Result: " + data.result;
}

function render(data) {
  renderArray(data.array);
  renderTree(data.tree);
}

function renderArray(array) {
  const container = document.getElementById("array");
  container.innerHTML = "";

  array.forEach(value => {
    const div = document.createElement("div");
    div.className = "array-item";
    div.innerText = value;
    container.appendChild(div);
  });
}

function renderTree(tree) {
  const container = document.getElementById("tree");
  container.innerHTML = "";

  const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  svg.classList.add("tree-svg");

  const nodes = [];

  let index = 0;
  let levelSize = 1;

  while (index < tree.length) {
    const levelDiv = document.createElement("div");
    levelDiv.className = "tree-level";

    for (let i = 0; i < levelSize && index < tree.length; i++) {
      const value = tree[index];

      const node = document.createElement("div");
      node.className = "node";
      node.innerText = value;

      levelDiv.appendChild(node);

      // ✅ store original index
      nodes.push({ index, el: node });

      index++;
    }

    container.appendChild(levelDiv);
    levelSize *= 2;
  }

  container.appendChild(svg);

  setTimeout(() => drawEdges(nodes, svg), 0);
}

function drawEdges(nodes, svg) {
  svg.innerHTML = "";

  const containerRect = svg.parentElement.getBoundingClientRect();

  // map index → element
  const nodeMap = new Map();
  nodes.forEach(n => nodeMap.set(n.index, n.el));

  for (let { index, el } of nodes) {
    const left = 2 * index + 1;
    const right = 2 * index + 2;

    if (nodeMap.has(left)) {
      drawLine(svg, el, nodeMap.get(left), containerRect);
    }

    if (nodeMap.has(right)) {
      drawLine(svg, el, nodeMap.get(right), containerRect);
    }
  }
}

function drawLine(svg, parent, child, containerRect) {
  const pRect = parent.getBoundingClientRect();
  const cRect = child.getBoundingClientRect();

  const x1 = pRect.left + pRect.width / 2 - containerRect.left;
  const y1 = pRect.bottom - containerRect.top;

  const x2 = cRect.left + cRect.width / 2 - containerRect.left;
  const y2 = cRect.top - containerRect.top;

  const line = document.createElementNS("http://www.w3.org/2000/svg", "line");

  line.setAttribute("x1", x1);
  line.setAttribute("y1", y1);
  line.setAttribute("x2", x2);
  line.setAttribute("y2", y2);

  line.setAttribute("stroke", "#999");
  line.setAttribute("stroke-width", "2");

  svg.appendChild(line);
}
