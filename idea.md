# Evolution

## The video
[video](https://www.youtube.com/watch?v=qwrp3lB-jkQ)

What they do:
- Simulate predators and prey moving around.
- They have a speed and a direction
- Predators and prays have energy that slowely runs down. If the energy is zero:
    - A predator dies
    - A prey stands still
- Pradators eat preys:
    - Predator gains energy and has a chance (or get points to this effect) to split
    - Prey dies
- Both see a small version of the world:
    - Preditors see directed
    - Preys see all around
- A neural network output angular momentum and speed
- grid is a torus


## Replication

We'll try to somewhat replicate the idea from the video. Without the gui at first. We can just report the number of preys vs predators.

## Simple first version

Make a grid and make the movement up, down, right, left on the grid. 

Predators can only look forward, prey in all directions.

Get a neural net to output the moving direction taking as input:
- The agents view of the world
- Its energy


# Alternative

maybe we can use a lot of the same structure, but instead of having moving prey, we just let one type of agent find food. Might make it easier to see progress.