using System;

namespace LooCast.Data.Runtime
{
    public interface IRuntimeDataSerializer
    {
        RuntimeData SerializableRuntimeData { get; }
    }
}
