class Stone {

    constructor(color, canvas, position) {
        this.color = color;
        this.position = position;
        this.canvas = canvas;
    }

    draw() {
        const circle = document.createElementNS(
            "http://www.w3.org/2000/svg",
            "circle"
        );

        x = this.position[0];
        y = this.position[1];

        circle.setAttribute("cx", x);
        circle.setAttribute("cy", y);
        circle.setAttribute("r", 12);
        circle.setAttribute("fill", this.color);
        circle.setAttribute("stroke", "black");
        
        svg.appendChild(circle);
    }

}

module.exports.Stone = Stone; 