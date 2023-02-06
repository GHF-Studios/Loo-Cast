using System;

namespace LooCast.Data.Runtime
{
    public interface IRuntimeDataDeserializer
    {
        RuntimeData SerializedRuntimeData { set; }
    }
}
