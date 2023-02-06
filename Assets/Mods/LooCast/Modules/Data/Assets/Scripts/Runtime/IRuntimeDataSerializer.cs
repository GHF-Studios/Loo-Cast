using System;

namespace LooCast.Data.Runtime
{
    public interface IRuntimeDataSerializer
    {
        RuntimeData SerializedRuntimeData { get; }
    }
}
