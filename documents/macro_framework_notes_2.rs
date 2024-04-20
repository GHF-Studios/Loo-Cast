1. Archetypes Module
2. Attributes Module
3. Commands Module
4. Components Module
5. Events Module
6. Primitives Module
7. Resources Module
8. States Module
9. Systems Module

1.: Archetypes
Archetypes are templates or blueprints for entities in Bevy. They are essentially just a collection of component types, 
    with some lightweight API wrapper for spawning an entity with exactly that set of components.
Beyond that, archetypes are not a relevant concept, as once the entity is spawned, 
    you should use systems to operate on certain components, 
    rather than a specific entity configuration as a whole, as is usual in an ECS context. 
Archetypes are really just a QOL feature, basically an archetype is just a way to directly associates a function,
    which can spawn an entity with the specified set of components, to the archetype itself.
    For Example: PlayerArchetype::spawn() would spawn an entity with the components "Player", "Transform", etc.  

2. Attributes Module
Handle meta-programming features that allow users to specify behavior or properties of components and systems declaratively.
A good example is the fact that any primitive is required to have the attribute "derive" with the parameters "clone", "send", and "sync".
Basically, a wrapper for rust's Attribute System, but limited to Components, Events, Primitives, Resources, and States, as these all represent data.
It's just a way to add attributes to types in a way that#s more locked down and specialized, and most importantly in a way, that the framework can understand and consider.

3. Commands Module
Manage and execute both CPU and GPU intensive tasks asynchronously through a unified API that abstracts away complex asynchronous patterns.
Basically, a wrapper for any sort of asynchronous CPU operation or any sort of (intrinsically asynchronous) GPU operation.
Commands have to explicitely define which inputs they take, which outputs they give, which error states could emerge, and what the actual executable operation code is.
A scheduler will analyze every defined command type for it's inputs and determine dependencies automatically,
    be it dependencies on certain pieces of Data defined by the framework.
The scheduler will also recognize bevy's concepts when used as input types, and also respect those dependencies; 
    e.g.: It will never schedule two commands into the same execution group when they both need the bevy resource "PhysicsConstants".
Essentially, any sort of deadlock or contention is to be eliminated either automatically by the way that bevy's ecs systems are optimized,
    or by the scheduler realizing that multiple operations need access to the same piece of framework-define data.
Basically, the commands module is a very complex wrapper for bevy systems,
    allowing us to both access bevy resources without needing to directly use/create a bevy ecs system, 
    and to access framework-defined data (and logic to some extent) in a way that respects dependencies on framework-defined data. 
Bevy resources, queries, states, etc. are specifically excluded from this, as bevy already handles these dependencies/potential deadlocks efficiently on it's own.

4. Components Module
Components are the building blocks of entities in Bevy. They are essentially just data types that are attached to entities.
This wrapper is just a way to define components in a way that the framework can understand and consider, 
    but more importantly, the definition of components is streamlined and requires minimal effort from a developer,
    just the most basic and essential information about a component, so the framework can recognize it for what it is, a managed bevy component type.
Managing/Tracking Component Types is important so that we not only get the scheduling benefits and dependency management of native bevy ecs systems,
    but also for the commands framework which is really just a function-like wrapper on top of bevy's ecs system that should respect
    inter-command dependencies and dependencies on non-primtive input paramaters (like component queries) which always need to be fetched and
    cannot be directly passed as input to a command, unlike primitives.

5. Events Module
The events module basically just wrapps bevy's events to be more flexible.
Basically, usually you could only use events by defining a system that takes an EventReader or EventWriter as a parameter.
But the wrapper will allow for these to also be used inside the commands module.
    When using this wrapper and specifying an EventReader or EventWriter as a parameter to a system,
        then the internal bevy event system will be used.
    But when these are supplied as parameters to a command,
        then the event system will not directly write to or read from bevy's event system,
        but does so via my event wrapper, allowing for the usage of events, outside of systems directly, for example inside commands.
Basically the events framework just facilitates the communication between the internal bevy event system and anywhere in the application where events are listened to or broadcast. 
This achieved by not allowing direct reading of events but rather by specifying a callback to be invoked every time (or only once, or three timnes, who knows) an event is broadcast and we want to "listen" to it.
Event broadcasting on the other hand is hardly different, as it's just a matter of calling some function to broadcast the event.
The real difference to bevy's event system is the fact that event listening is handled by just attaching (and potentially detaching) a callback(aka an event listener) to an event.

6. Primitives Module
A primitive is just any type that's not created or managed by the framework.
Every primitive must have the derive attributes of "clone", "send", and "sync".
When a primitive type is used as a type of input to a command, then there will be no dependency checking of any kind regarding the primitives,
    or any scheduling considerations. A primitive parameter must immediately be passed to a command when requesting that command.
    It's essentially also fetched like other input types, but from the user, not from bevy's ECS, hence the clarification.

7. Resources Module
Resources are global data that can be accessed from anywhere in the application.
The resources module is just a way to define resources in a way that the framework can understand and consider,
    but more importantly, the definition of resources is streamlined and requires minimal effort from a developer,
    just the most basic and essential information about a resource, so the framework can recognize it for what it is, a managed bevy resource type.
Managing/Tracking Resource Types is important so that we not only get the scheduling benefits and dependency management of native bevy ecs systems,
    but also for the commands framework which is really just a function-like wrapper on top of bevy's ecs system that should respect
    inter-command dependencies and dependencies on non-primtive input paramaters (like resource queries) which always need to be fetched and
    cannot be directly passed as input to a command, unlike primitives.

8. States Module

9. Systems Module
The bevy ecs systems wrapper, or systems framework, is surprisingly simple, even though it wrapps such an integral aspect of bevy's ECS.
It adds nothing new to bevy systems, but it allows us to create bevy systems in a very streamlined and declarative way,
    and it allows us to track the exact composition of a system's input parameters.