function main_setup()
    ops = {}
    ops.compiledPrimitives = {}
    ops.primitiveBindings = {}
    ops.compositeDefinitions = {}
    ops.callbacks = {}

    function ops::define_composite(composite_id, entry_callback_id)
        if ops.compositeDefinitions[composite_id] ~= nil then
            error("Composite operation '" .. composite_id .. "' already defined!")
        end

        ops.compositeDefinitions[composite_id] = callback_id
    end

    function ops::register_callback()
        local callback_id = #ops.callbacks + 1
        ops.callbacks[callback_id] = nil
        return callback_id
    end

    function ops::load_callback(callback_id, callback_function)
        if ops.callbacks[callback_id] == nil then
            error("Callback '" .. callback_id .. "' not registered!")
        end

        if type(callback_function) ~= "function" then
            error("Callback '" .. callback_id .. "' must be a function!")
        end

        ops.callbacks[callback_id] = callback_function
    end

    function ops::invoke_primitive(primitive_binding_id, callback_id, ...)
        local primitive_bind_name = ops.primitiveBindings[primitive_operation_id]
        if primitive_bind_name == nil then
            error("Primitive operation '" .. primitive_operation_id .. "' not bound!")
        end

        local primitive = ops.compiledPrimitives[primitive_operation_id]
        if primitive == nil then
            error("Primitive operation '" .. primitive_operation_id .. "' not compiled!")
        end

        primitive(callback_id, ...)
    end

    function ops::invoke_composite(composite_id, callback_id, ...)
        local composite_function = ops.compositeDefinitions[composite_id]
        if composite_function == nil then
            error("Composite operation '" .. composite_id .. "' not defined!")
        end

        composite_function(ops, ...)
    end

    function ops::invoke_callback(callback_id, ...)
        if ops.callbacks[callback_id] == nil then
            error("Callback '" .. callback_id .. "' not defined!")
        end

        local callback_function = ops.callbacks[callback_id]
        callback_function(...)
    end

    _G["ops"] = ops

    testOps = {}

    function testOps::setupTest()
        local step0_id = ops::register_callback()
        ops::load_callback(step0_id, function(ops, chunk_id)
            ops::invoke_primitive("entity.create", step1_id)
        end)

        local step1_id = ops::register_callback()
        ops::load_callback(step1_id, function(ops, entity_id, chunk_id)
            ops::invoke_primitive("chunk.upgrade", step2_id, entity_id, chunk_id)
        end)

        local step2_id = ops::register_callback()
        ops::load_callback(step2_id, function(ops, entity_id, chunk_id)
            print("Created chunk '" .. chunk_id .. "' entity '" .. entity_id .. "'!")
        end)

        ops::define_composite("example.spawnChunk", step0_id)
    end

    function testOps::runTest()
        ops::invoke_composite("example.spawnChunk", chunk_id)
    end

    _G["testOps"] = testOps
end

initialize_ops_module()

