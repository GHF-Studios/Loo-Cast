function main_setup()
    ops = {}
    ops.compiledPrimitives = {}
    ops.primitiveBindings = {}
    ops.compositeDefinitions = {}
    ops.steps = {}

    function ops::define_composite(composite_id, entry_step_id)
        if ops.compositeDefinitions[composite_id] ~= nil then
            error("Composite operation '" .. composite_id .. "' already defined!")
        end

        ops.compositeDefinitions[composite_id] = step_id
    end

    function ops::register_step()
        local step_id = #ops.steps + 1
        ops.steps[step_id] = nil
        return step_id
    end

    function ops::load_step(step_id, step_function)
        if ops.steps[step_id] == nil then
            error("step '" .. step_id .. "' not registered!")
        end

        if type(step_function) ~= "function" then
            error("step '" .. step_id .. "' must be a function!")
        end

        ops.steps[step_id] = step_function
    end

    function ops::invoke_primitive(primitive_binding_id, callback_step_id, ...)
        local primitive_bind_name = ops.primitiveBindings[primitive_operation_id]
        if primitive_bind_name == nil then
            error("Primitive operation '" .. primitive_operation_id .. "' not bound!")
        end

        local primitive = ops.compiledPrimitives[primitive_operation_id]
        if primitive == nil then
            error("Primitive operation '" .. primitive_operation_id .. "' not compiled!")
        end

        primitive(callback_step_id, ...)
    end

    function ops::invoke_composite(composite_id, callback_step_id, ...)
        local composite = ops.compositeDefinitions[composite_id]
        if composite == nil then
            error("Composite operation '" .. composite_id .. "' not defined!")
        end

        composite(ops, callback_step_id, ...)
    end

    function ops::invoke_step(step_id, ...)
        if ops.steps[step_id] == nil then
            error("step '" .. step_id .. "' not defined!")
        end

        local step = ops.steps[step_id]
        step(...)
    end

    ops.NOSTEP = ops::register_step()
    
    _G["ops"] = ops

    test = {}

    function test::define_showcase_example()
        local step0 = ops::register_step()
        local step1 = ops::register_step()
        local step2 = ops::register_step()
        local step3 = ops::register_step()

        ops::load_step(step0, function(ops, callback_step_id, entity_id)
            ops::invoke_primitive("entity.move", step1, entity_id, 0, 0, 0)
        end)

        ops::load_step(step1, function(ops, callback_step_id, entity_id)
            ops::invoke_primitive("entity.rotate", step2, entity_id, 0, 0, 0)
        end)

        ops::load_step(step2, function(ops, callback_step_id, entity_id)
            ops::invoke_primitive("entity.scale", step3, entity_id, 1, 1, 1)
        end)

        ops::load_step(step3, function(ops, callback_step_id, entity_id)
            print("Entity '" .. entity_id .. "' moved, rotated, and scaled!")
            ops::invoke_step(callback_step_id)
        end)

        ops::define_composite("example.showcase", step0)
    end

    function test::define_spawn_chunk()
        local step0 = ops::register_step()
        local step1 = ops::register_step()
        
        ops::load_step(step0, function(ops, chunk_id)
            ops::invoke_primitive("entity.create", step1)
        end)

        ops::load_step(step1, function(ops, entity_id, chunk_id)
            ops::invoke_primitive("chunk.upgrade", ops.NOSTEP, entity_id, chunk_id)
        end)

        ops::define_composite("example.spawn_chunk", step0)
    end

    function test::define_despawn_chunk()
        local step0 = ops::register_step()
        ops::load_step(step0, function(ops, entity_id)
            ops::invoke_primitive("chunk.downgrade", step1)
        end)

        local step1 = ops::register_step()
        ops::load_step(step1, function(ops, entity_id)
            ops::invoke_primitive("entity.destroy", ops.NOSTEP)
        end)

        ops::define_composite("example.despawnChunk", step0)
    end

    function test::define_start_timer()
        local step0 = ops::register_step()
        ops::load_step(step0, function(ops, timer_id, duration)
            ops::invoke_primitive("timer.start", step1, timer_id, duration)
        end)

        local step1 = ops::register_step()
        ops::load_step(step1, function(ops, timer_id)
            print("Timer '" .. timer_id .. "' started!")
        end)

        ops::define_composite("example.startTimer", step0)
    end

    function test::define_stop_timer()
        local step0 = ops::register_step()
        ops::load_step(step0, function(ops, timer_id)
            ops::invoke_primitive("timer.stop", step1, timer_id)
        end)

        local step1 = ops::register_step()
        ops::load_step(step1, function(ops, timer_id, elapsed_time)
            print("Timer '" .. timer_id .. "' stopped!")
        end)

        ops::define_composite("example.stopTimer", step0)
    end

    function test::runTest()
        ops::invoke_composite("example.spawnChunk", chunk_id)
        ops::invoke_composite("example.despawnChunk", chunk_id)
    end

    _G["test"] = test
end

main_setup()