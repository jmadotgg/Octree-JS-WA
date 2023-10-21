class Point {
    constructor(x, y, z) {
        this.x = x;
        this.y = y;
        this.z = z;
    }
}

class OctreeNode {
    constructor(center, size) {
        this.center = center;
        this.size = size;
        this.points = [];
        this.children = null;
    }

    insert(point) {
        if (this.size <= 0.1) {
            this.points.push(point);
            return;
        }

        if (!this.children) {
            this.subdivide();
        }

        const octant = this.getOctant(point);
        this.children[octant].insert(point);
    }

    subdivide() {
        const halfSize = this.size / 2;
        const x = this.center.x;
        const y = this.center.y;
        const z = this.center.z;

        this.children = [
            new OctreeNode(new Point(x - halfSize, y - halfSize, z - halfSize), halfSize),
            new OctreeNode(new Point(x - halfSize, y - halfSize, z + halfSize), halfSize),
            new OctreeNode(new Point(x - halfSize, y + halfSize, z - halfSize), halfSize),
            new OctreeNode(new Point(x - halfSize, y + halfSize, z + halfSize), halfSize),
            new OctreeNode(new Point(x + halfSize, y - halfSize, z - halfSize), halfSize),
            new OctreeNode(new Point(x + halfSize, y - halfSize, z + halfSize), halfSize),
            new OctreeNode(new Point(x + halfSize, y + halfSize, z - halfSize), halfSize),
            new OctreeNode(new Point(x + halfSize, y + halfSize, z + halfSize), halfSize),
        ];

        for (const point of this.points) {
            const octant = this.getOctant(point);
            this.children[octant].insert(point);
        }

        this.points = [];
    }

    getOctant(point) {
        const x = point.x >= this.center.x ? 1 : 0;
        const y = point.y >= this.center.y ? 1 : 0;
        const z = point.z >= this.center.z ? 1 : 0;
        return (x << 2) | (y << 1) | z;
    }

    queryPoint(point) {
        if (this.size <= 0.1) {
            for (const p of this.points) {
                if (p.x === point.x && p.y === point.y && p.z === point.z) {
                    return p;
                }
            }
        } else if (this.children) {
            const octant = this.getOctant(point);
            return this.children[octant].queryPoint(point);
        }

        return null;
    }

    //traverse(callback) {
    //    callback(this);

    //    if (this.children) {
    //        for (const child of this.children) {
    //            child.traverse(callback);
    //        }
    //    }
    //}
}

export function runJs(size) {
    const root = new OctreeNode(new Point(0, 0, 0), 10);

    for (let i = 0; i < size; i++) {
        root.insert(Math.random() * size, Math.random() * size, Math.random() * size);
    }
    const queryPoint = new Point(0.444 * size, 0.666 * size, 0.888 * size);

    root.insert(queryPoint);

    const foundPoint = root.queryPoint(queryPoint);

    if (foundPoint) {
        return `Point (${foundPoint.x}, ${foundPoint.y}, ${foundPoint.z})`;
    } else {
        return "Point not found.";
    }
}
