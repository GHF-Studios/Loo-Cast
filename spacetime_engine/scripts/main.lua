function initialize_ops_module()
    ops = {}
    ops.primitiveBindings = {}
    ops.compositeDefinitions = {}

    -- Bind a Lua name to a single Rust function
    function ops:bindPrimitive(luaPrimitiveID, rustFunction)
        if self.primitiveBindings[luaPrimitiveID] then
            error("A primitive is already bound to '" .. luaPrimitiveID .. "'. Unbind it first.")
        end
        self.primitiveBindings[luaPrimitiveID] = rustFunction
    end

    -- Unbind a Lua name from its Rust function
    function ops:unbindPrimitive(luaPrimitiveID)
        self.primitiveBindings[luaPrimitiveID] = nil
    end

    -- Invoke a Rust function through its Lua binding
    function ops:invokePrimitive(luaPrimitiveID, ...)
        local rustFunction = self.primitiveBindings[luaPrimitiveID]
        if rustFunction then
            return rustFunction(...)
        else
            error("Primitive binding '" .. luaPrimitiveID .. "' not found!")
        end
    end

    -- Define a composite function that combines primitives or other composites
    function ops:defineComposite(compositeID, compositeFunction)
        self.compositeDefinitions[compositeID] = compositeFunction
    end

    -- Undefine a composite function
    function ops:undefineComposite(compositeID)
        self.compositeDefinitions[compositeID] = nil
    end

    -- Invoke a composite function
    function ops:invokeComposite(compositeID, ...)
        local compositeFunction = self.compositeDefinitions[compositeID]
        if compositeFunction then
            return compositeFunction(self, ...)
        else
            error("Composite definition '" .. compositeID .. "' not found!")
        end
    end
end

-- Register the composite inline
ops:defineComposite("add_and_multiply", function(self, x, y, z)
    local sum = self:invokePrimitive('add_integers', x, y)
    return self:invokePrimitive('multiply_integers', sum, z)
end)
