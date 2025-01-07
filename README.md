# CellSimulation
## How the Simulation Works

The Cell Simulation models the behavior of cells in a dynamic environment. This section provides an in-depth explanation of the various components and processes involved in the simulation.

### Overview

The simulation consists of cells that interact with their environment and each other. Each cell has a unique set of attributes and behaviors defined by its DNA. The environment contains food sources that cells can consume to survive and replicate. The simulation runs in iterations, where each iteration represents a single time step in the simulation.

### Components

#### Cells

Cells are the primary entities in the simulation. Each cell has the following attributes:
- **DNA**: Defines the cell's behavior and attributes.
- **Position**: The current coordinates of the cell in the environment.
- **Food**: The amount of food the cell has, which is necessary for survival and replication.
- **Size**: The size of the cell, which affects its food consumption and movement.

Cells can perform various actions such as moving, consuming food, emitting forces, and replicating. The behavior of each cell is influenced by its DNA.

#### DNA

The DNA of a cell is a vector of codons, where each codon is a tuple consisting of:
- **Primary Base**: Defines the type of codon (e.g., attraction, emission, mutation rates).
- **Secondary Base**: Defines the target of the codon (e.g., specific forces).
- **Value**: Defines the magnitude or rate of the codon.

The DNA determines the cell's behavior, including its attractions, emissions, mutation rates, and replication requirements. The DNA can mutate over time, leading to changes in the cell's behavior.

Collecting workspace information

#### DNA Primary Bases

In the Cell Simulation, each cell's behavior is determined by its DNA, which consists of a series of codons. Each codon is a tuple containing a primary base, a secondary base, and a value. The primary base defines the type of codon and its effect on the cell's behavior. Here is an explanation of each primary base:

##### 1. Attraction (0)
- **Description**: This codon type defines the attraction forces that influence the cell's movement towards specific targets.
- **Effect**: The cell will be attracted to the target defined by the secondary base with a magnitude specified by the value.

##### 2. Emission (1)
- **Description**: This codon type defines the emission forces that the cell emits to influence other cells.
- **Effect**: The cell emits a force defined by the secondary base with a magnitude specified by the value. This can affect nearby cells, either attracting or repelling them.

##### 3. Disable Codon (2)
- **Description**: This codon type can disable other codons based on certain conditions.
- **Effect**: If the force defined by the secondary base exceeds the value, the codon is disabled, preventing its effect on the cell's behavior.

##### 4. Global Mutation Rate (3)
- **Description**: This codon type defines the global mutation rate for the cell's DNA.
- **Effect**: The value specifies the rate at which mutations occur in the cell's DNA.

##### 5. Individual Mutation Rate (4)
- **Description**: This codon type defines the mutation rate for individual codons.
- **Effect**: The value specifies the mutation rate for the codon defined by the secondary base.

##### 6. Primary Mutation Rate (5)
- **Description**: This codon type defines the mutation rate for the primary base of codons.
- **Effect**: The value specifies the rate at which the primary base of codons mutates.

##### 7. Secondary Mutation Rate (6)
- **Description**: This codon type defines the mutation rate for the secondary base of codons.
- **Effect**: The value specifies the rate at which the secondary base of codons mutates.

##### 8. Add Codon Mutation Rate (7)
- **Description**: This codon type defines the rate at which new codons are added to the DNA.
- **Effect**: The value specifies the rate at which new codons are added to the cell's DNA.

##### 9. Remove Codon Mutation Rate (8)
- **Description**: This codon type defines the rate at which codons are removed from the DNA.
- **Effect**: The value specifies the rate at which codons are removed from the cell's DNA.

##### 10. Replication Food (9)
- **Description**: This codon type defines the amount of food required for the cell to replicate.
- **Effect**: The value specifies the amount of food the cell needs to accumulate before it can replicate.

##### 11. Cell Size (10)
- **Description**: This codon type defines the size of the cell.
- **Effect**: The value specifies the size of the cell, which affects its food consumption and movement.

Each primary base plays a crucial role in determining the cell's behavior and interactions within the simulation. The combination of these codons in the DNA creates a diverse range of behaviors and evolutionary possibilities for the cells.

#### Environment

The environment is a grid where cells and food sources are placed. The grid is divided into smaller cells, each containing a set of cells and food sources. The environment manages the placement and movement of cells and food sources, as well as the interactions between them.

### Processes

#### Initialization

The simulation starts by initializing the environment and creating a set of starting cells and food sources. Cells are placed at random positions with default DNA, and food sources are distributed randomly in the environment.

#### Iteration

Each iteration of the simulation consists of the following steps:
1. **Emit Forces**: Cells emit forces based on their DNA, which affect the movement of nearby cells. For example, cells may emit food forces that attract other cells.
2. **Update Cells**: Each cell updates its position and food based on the forces applied to it. Cells consume food to move and emit forces.
3. **Attempt to Eat**: Cells attempt to consume nearby food sources. If a cell successfully consumes food, the food source is removed from the environment.
4. **Replication**: Cells check if they have enough food to replicate. If a cell can replicate, it creates a new cell with mutated DNA and resets its own state.
5. **Add Food**: New food sources are added to the environment at random positions.

#### Forces

Forces play a crucial role in the simulation. They determine how cells interact with each other and their environment. There are different types of forces, such as:
- **Attraction Forces**: Attract cells towards specific targets (e.g., food).
- **Emission Forces**: Emitted by cells to influence nearby cells (e.g., toxins).

Cells process these forces based on their DNA, which defines how they respond to different types of forces.

Collecting workspace information

## Build and Run Instructions

### Building the Project

To build the project, you can use the following command:

```sh
cargo build --release
```

This will compile the project in release mode, optimizing for performance.

### Running the Simulation

To run the simulation, use the following command:

```sh
cargo run --release
```

This will execute the simulation with the default configuration.

### Using Features

The project includes optional features that can be enabled during build and run:

- **Graphics**: Enables graphical representation of the simulation using the `minifb` crate.
- **Profiling**: Enables profiling capabilities using the `pprof` crate.

To enable these features, use the `--features` flag. For example, to enable both graphics and profiling:

```sh
cargo run --release --features "graphics profiling"
```

### Modifying the Configuration

The simulation configuration is defined in the `config.rs` file. Here are some key configuration parameters you can modify:

- **Save Files**:
    - `STATE_PATH`: The path of the file to save and load the state from.

- **Window Dimensions**:
    - `WINDOW_WIDTH`: Width of the simulation window.
    - `WINDOW_HEIGHT`: Height of the simulation window.
    - `SLEEP_TIME`: Time to sleep between each rendered frame.

- **Simulation Parameters**:
    - `ITERATIONS`: Number of iterations the simulation will run.
    - `PRINT_DETAILS_AFTER_FRAMES`: Number of frames after which details are printed.

- **Game Size and Grid**:
  - `GAME_SIZE`: Size of the game environment.
  - `GRID_CELL_SIZE`: Size of each grid cell.

- **Starting Conditions**:
    - `STARTING_CELLS`: Number of starting cells.
    - `STARTING_FOOD`: Number of starting food sources.

- **Food**:
    - `DEFAULT_FOOD_VALUE`: Default value of food.
    - `DEFAULT_CELL_FOOD_VALUE`: Default value of food for a dead cell.
    - `FOOD_ADDED_PER_FRAME`: Number of food sources added per frame. 
    - `MAX_FOOD`: Max number of food sources.

- **Forces**:
    - `FORCE_MAX_RANGE_SQ`: Maximum range of forces (squared). This must be less than or equal to sqrt(`GRID_CELL_SIZE`)
    - `FOOD_FORCE`: The ID for the food force.
    - `TOXIN_FORCE`: The ID for the toxin force.

- **Cell Attributes**:
    - `CELL_STARTING_FOOD`: Starting amount of food for each cell.
    - `MAX_CELLS`: Maximum amount of cells allowed.
 
- **Cell DNA Defaults**:
    - `DEFAULT_MUTATION_RATE`: Default global mutation rate.
    - `DEFAULT_PRIMARY_MUTATION_RATE`: Default mutation rate for primary bases.
    - `DEFAULT_SECONDARY_MUTATION_RATE`: Default mutation rate for secondary bases.
    - `DEFAULT_ADD_CODON_MUTATION_RATE`: Default rate for adding new codons.
    - `DEFAULT_REMOVE_CODON_MUTATION_RATE`: Default rate for removing codons.
    - `DEFAULT_FOOD_TO_REPLICATE`: Default amount of food required for replication.
    - `DEFAULT_CELL_SIZE_SQ`: Default size of cells (squared).

- **Food Usage**:
    - `FOOD_USED_PER_FRAME`: Food used per frame.
    - `FOOD_STOLEN_PER_TOXIN_UNIT`: Food stolen per toxin unit.
    - `FOOD_USED_PER_UNIT_MOVED`: Food used per unit moved.
    - `FOOD_USED_PER_SIZE_UNIT`: Food used per size unit.
    - `FOOD_USED_PER_FORCE_EMITTED`: Food used per force emitted.
    - `FOOD_USED_PER_TOXIN_UNIT_EMITTED`: Food used per toxin unit emitted.
    - `FOOD_RETENTION_FROM_REPLICATION`: Food retention after replication.

To modify any of these parameters, simply edit the values in the `config.rs` file and rebuild the project.

## Conclusion

The Cell Simulation project is a detailed and complex system that models the behavior of cells in a simulated environment. The simulation involves various components and processes, including cells, DNA, the environment, initialization, iteration, and forces. Each component plays a crucial role in the overall behavior of the simulation, making it a fascinating and intricate system to study and explore.
