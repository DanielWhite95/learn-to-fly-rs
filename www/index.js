import * as simulationWasm from 'lib-simulation-wasm';

let sim = null;
let drawing = true;
const generationNumberOutput = document.getElementById("generation-number")
const generationAgeOutput = document.getElementById("generation-age")
const generationScoreOutput = document.getElementById("generation-score")

function drawAnimal(ctx, animal, canvasWidth, canvasHeight) {
    ctx.fillStyle = "rgb(255, 0, 0)";
    ctx.fillRect(animal.x * canvasWidth , animal.y * canvasHeight, 10, 10);
}

function drawFood(ctx, food, canvasWidth, canvasHeight) {
    ctx.fillStyle = "rgb(0, 255, 0)";
    ctx.fillRect(food.x * canvasWidth, food.y * canvasHeight, 10, 10);
}

function drawFrame() {
    if (!drawing) {
         return window.requestAnimationFrame(drawFrame)
    }
    const canvas = document.getElementById("simulation-viewport");
    const canvasHeight = canvas.height;
    const canvasWidth = canvas.width;
    const ctx = canvas.getContext("2d");
    if (!ctx) {
        console.error("Cannot load canvas for simulation");
        throw new Exception("Cannot load canvas")
    }
    ctx.clearRect(0, 0, canvasWidth, canvasHeight); // clear canvas
    const world = sim.world();
    for(const animal of world.animals) {
        drawAnimal(ctx, animal, canvasWidth, canvasHeight)
    };
    for(const food of world.food) {
        drawFood(ctx, food, canvasWidth, canvasHeight)
    }
    const stats = sim.step();
    generationAgeOutput.textContent = Number(generationAgeOutput.textContent) + 1;
    if (stats) {
        generationAgeOutput.textContent = 0;
        generationNumberOutput.textContent = Number(generationNumberOutput.textContent) + 1;
        generationScoreOutput.textContent = stats.avg_score;
    }
    window.requestAnimationFrame(drawFrame)
};

function skipGeneration() {
    drawing = false;
    let skippedGeneration = 0;
    while (skippedGeneration < 10) {
        const stats = sim.step();
        if (stats) {
            skippedGeneration++
            generationNumberOutput.textContent = Number(generationNumberOutput.textContent) + 1;
        };
    };
    generationAgeOutput.textContent = 0;
    drawing = true;

}

function startSimulation(animals, food, mutRate, mutCoeff) {
    sim = new simulationWasm.Simulation(animals, food, mutRate, mutCoeff, 500);
    generationAgeOutput.textContent = "0";
    generationNumberOutput.textContent = "1";
    generationScoreOutput.textContent = "0.00"
}

function setupCanvas() {
    const canvas = document.getElementById("simulation-viewport");
    const dpr = window.devicePixelRatio || 1;
    const rect = canvas.parentNode.getBoundingClientRect();
    const canvasWidth = rect.width;
    const canvasHeight = canvasWidth / 16 * 9;
    // Set the "actual" size of the canvas
    canvas.width = canvasWidth * dpr;
    canvas.height = canvasHeight * dpr;


    // Set the "drawn" size of the canvas
    canvas.style.width = `${canvasWidth}px`;
    canvas.style.height = `${canvasHeight}px`;

    const ctx = canvas.getContext("2d");
    if (!ctx) {
        console.error("Cannot load canvas for simulation");
        throw new Exception("Cannot load canvas")
    }
    ctx.scale(dpr, dpr);
    // Scale the context to ensure correct drawing operations
    ctx.globalCompositeOperation = "destination-over";
}



function init () {
    const simulationAnimals = document.getElementById("simulation-animals");
    const simulationFood = document.getElementById("simulation-food");
    const simulationMutationRate = document.getElementById("simulation-mut-rate");
    const simulationMutationCoefficient = document.getElementById("simulation-mut-coeff");
    const fastForwardButton = document.getElementById("fast-forward-button");


    const simulationAnimalsOutput = document.getElementById("simulation-animals-value");
    simulationAnimalsOutput.textContent = simulationAnimals.value
    const simulationFoodOutput = document.getElementById("simulation-food-value");
    simulationFoodOutput.textContent = simulationFood.value
    const simulationMutationCoefficientOutput = document.getElementById("simulation-mutation-coefficient-value");
    simulationMutationCoefficientOutput.textContent = simulationMutationCoefficient.value
    const simulationMutationRateOutput = document.getElementById("simulation-mutation-rate-value");
    simulationMutationRateOutput.textContent = simulationMutationRate.value

    simulationAnimals.addEventListener("input", (event) => {
        startSimulation(
            simulationAnimals.value,
            simulationFood.value,
            simulationMutationRate.value,
            simulationMutationCoefficient.value
        );
        simulationAnimalsOutput.textContent = event.target.value;
    });
    simulationFood.addEventListener("input", (event) => {
        startSimulation(
            simulationAnimals.value,
            simulationFood.value,
            simulationMutationRate.value,
            simulationMutationCoefficient.value
        );
        simulationFoodOutput.textContent = event.target.value
    });
    simulationMutationRate.addEventListener("input", (event) => {
        startSimulation(
            simulationAnimals.value,
            simulationFood.value,
            simulationMutationRate.value,
            simulationMutationCoefficient.value
        );
        simulationMutationRateOutput.textContent = event.target.value
    });
    simulationMutationCoefficient.addEventListener("input", (event) => {
        startSimulation(
            simulationAnimals.value,
            simulationFood.value,
            simulationMutationRate.value,
            simulationMutationCoefficient.value
        );
        simulationMutationCoefficientOutput.textContent = event.target.value
    });
    fastForwardButton.addEventListener("click", (event) => {
        skipGeneration()
    });


    startSimulation(
            simulationAnimals.value,
            simulationFood.value,
            simulationMutationRate.value,
            simulationMutationCoefficient.value
    );
    setupCanvas();
    window.addEventListener("resize", (event) => {
        setupCanvas();
    });
    window.requestAnimationFrame(drawFrame);
}

init();
