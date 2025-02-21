'use strict';

process.stdin.resume();
process.stdin.setEncoding('utf-8');

let inputString: string = '';
let inputLines: string[] = [];
let currentLine: number = 0;

process.stdin.on('data', function (inputStdin: string): void {
    inputString += inputStdin;
});

process.stdin.on('end', function (): void {
    inputLines = inputString.trim().split('\n');
    main();
});

function readLine(): string {
    return inputLines[currentLine++];
}

class Nodee {
    data: number;
    left: Nodee | null;
    right: Nodee | null;

    constructor(data: number) {
        this.data = data;
        this.left = null;
        this.right = null;
    }
}

function buildTree(values: number[]): Nodee | null {
    if (values.length === 0) return null;

    let root = new Nodee(values[0]);
    let queue: Nodee[] = [root];
    let i = 1;

    while (i < values.length) {
        let current = queue.shift()!;
        
        if (i < values.length && values[i] !== -1) {
            current.left = new Nodee(values[i]);
            queue.push(current.left);
        }
        i++;

        if (i < values.length && values[i] !== -1) {
            current.right = new Nodee(values[i]);
            queue.push(current.right);
        }
        i++;
    }
    return root;
}

function preOrder(root: Nodee | null) {
    if (root === null) {
        return;
    }

    process.stdout.write(root.data + " ");
    preOrder(root.left);
    preOrder(root.right);
}

function main() {
    let values = readLine().split(' ').map(Number);
    let root = buildTree(values);
    preOrder(root);
}
