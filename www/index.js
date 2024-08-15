import * as simulationWasm from 'lib-simulation-wasm';

function drawAnimal(ctx, animal, canvasWidth, canvasHeight) {
    ctx.fillStyle = "rgb(255, 0, 0)";
    ctx.fillRect(animal.x * canvasWidth, animal.y * canvasHeight, 10, 10);
}

function drawFood(ctx, food, canvasWidth, canvasHeight) {
    ctx.fillStyle = "rgb(0, 255, 0)";
    ctx.fillRect(food.x * canvasWidth, food.y * canvasHeight, 10, 10);
}

function drawFrame() {
    const canvas = document.getElementById("simulation-viewport");
    const canvasHeight = canvas.height;
    const canvasWidth = canvas.width;
    const ctx = canvas.getContext("2d");
    if (!ctx) {
        console.error("Cannot load canvas for simulation");
        throw new Exception("Cannot load canvas")
    }

    ctx.globalCompositeOperation = "destination-over";
    ctx.clearRect(0, 0, canvasWidth, canvasHeight); // clear canvas

    const world = sim.world();
    for(const animal of world.animals) {
        drawAnimal(ctx, animal, canvasWidth, canvasHeight)
    };
    for(const food of world.food) {
        drawFood(ctx, food, canvasWidth, canvasHeight)
    }
    sim.step();
    window.requestAnimationFrame(drawFrame)
};

function startSimulation() {
    sim = new simulationWasm.Simulation(simulationAnimals.value, simulationFood.value, simulationMutationRate.value, simulationMutationCoefficient.value)
}

const simulationAnimals = document.getElementById("simulation-animals");
const simulationFood = document.getElementById("simulation-food");
const simulationMutationRate = document.getElementById("simulation-mut-rate");
const simulationMutationCoefficient = document.getElementById("simulation-mut-coeff");

const simulationAnimalsOutput = document.getElementById("simulation-animals-value");
simulationAnimalsOutput.textContent = simulationAnimals.value
const simulationFoodOutput = document.getElementById("simulation-food-value");
simulationFoodOutput.textContent = simulationFood.value
const simulationMutationCoefficientOutput = document.getElementById("simulation-mutation-coefficient-value");
simulationMutationCoefficientOutput.textContent = simulationMutationCoefficient.value
const simulationMutationRateOutput = document.getElementById("simulation-mutation-rate-value");
simulationMutationRateOutput.textContent = simulationMutationRate.value

simulationAnimals.addEventListener("input", (event) => {
    startSimulation();
    simulationAnimalsOutput.textContent = event.target.value;
});
simulationFood.addEventListener("input", (event) => {
    startSimulation();
    simulationFoodOutput.textContent = event.target.value
});
simulationMutationRate.addEventListener("input", (event) => {
    startSimulation();
    simulationMutationRateOutput.textContent = event.target.value
});
simulationMutationCoefficient.addEventListener("input", (event) => {
    startSimulation();
    simulationMutationCoefficientOutput.textContent = event.target.value
});



let sim = null;
startSimulation();
window.requestAnimationFrame(drawFrame);
